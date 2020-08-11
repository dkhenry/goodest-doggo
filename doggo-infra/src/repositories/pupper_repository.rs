use mysql;
use doggo_core::dtos::Pupper;
use super::CLIENT_POOL;
use super::Pool;
use indexmap::IndexMap;
use doggo_core::collection_abstractions::PupperRepository;
use rand::Rng;
use mysql::AccessMode;
use mysql::IsolationLevel;
use mysql::prelude::Queryable;

const RETRY_COUNT: i32 = 5;

pub struct VitessPupperRepository {
    pool: Pool,
}

impl VitessPupperRepository {
    /// Associative function to create a new query handler from a connection.
    pub fn new() -> VitessPupperRepository {
        VitessPupperRepository {
            // "Clone" the pool (it's an Arc, so just increase count) and then get a connection for use
            // in this handler.
            pool: CLIENT_POOL.clone(),
        }
    }
    // May optionally return a rating upon successful db interaction.  Underlying db
    // error will be communicated as a mysql::Error in the Err result variant returned.
    fn puppers_rating(&mut self, id: u64) -> Result<Option<f64>, mysql::Error> {
        self.pool.query_first(
            format!(r"SELECT COALESCE(SUM(r.rating)/COUNT(r.rating),0.0)
            FROM puppers@replica.ratings as r
            WHERE r.pupper_id='{}'", id)
        )
    }

    fn pup_with_rating(
        &mut self,
        pup_data: (u64, String, String)
    ) -> Result<Option<Pupper>, mysql::Error> {
        let maybe_rating = self.puppers_rating(pup_data.0)?;
        return Ok(
            Some(
                Pupper {
                    id: pup_data.0,
                    name: pup_data.1,
                    image: pup_data.2,
                    rating: maybe_rating,
                }
            )
        );
    }

    fn puppers_from_rating_list(
        &mut self,
        list: Vec<(u64, f64)>,
    ) -> Result<Vec<Pupper>, mysql::Error> {
        // Cloning primitives is cheap, so doing so is virtually cost free for such a small list.
        // Using IndexMap because it maintains insertion order.
        let mut pupper_map: IndexMap<u64, Pupper> = list.iter().map(|(i, r)| {
            (i.clone(), Pupper {
                id: i.clone(),
                name: "".to_string(),
                image: "".to_string(),
                rating: Some(*r)
            })
        }).collect();

        let ids_str: String = list.iter().map(|(i, _)| i.to_string()).collect::<Vec<String>>().join("','");

        self.pool.get_conn().unwrap().query_iter(
            format!(r"SELECT p.id as id, p.name as name, p.image as image
            FROM puppers AS p
            WHERE id in
            ('{}')", ids_str)
        ).map(|result| {
            result.for_each(|row_result| {
                if let Ok(r) = row_result {
                    let (id, name, image) = mysql::from_row(r);
                    pupper_map.entry(id).and_modify(|p| { p.name = name; p.image = image});
                }
            })
        })?;

        let puppers = pupper_map.into_iter().map(|(_, p)| p).collect();

        Ok(puppers)
    }
}

impl PupperRepository for VitessPupperRepository {
    type Error = mysql::Error;

    fn get(&mut self, pupper_id: u64) -> Result<Option<Pupper>, mysql::Error> {
        let r: Option<(u64, String, String)> = self.pool.query_first(
            format!(r"SELECT p.id, p.name, p.image
            FROM puppers AS p
            WHERE p.id = {}", pupper_id)
        )?;

        if let Some(pup) = r {
            return self.pup_with_rating(pup);
        }

        // Didn't find a pupper :-(
        Ok(None)
    }

    fn get_random(&mut self) -> Result<Option<Pupper>, mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        let mut tx = match conn.start_transaction(
            mysql::TxOpts::default()
                .set_isolation_level(Some(IsolationLevel::ReadCommitted))
                .set_access_mode(Some(AccessMode::ReadOnly))
            ) {
            Ok(v) => v,
            Err(e) => {
                self.pool.set_is_working(false);
                return Err(e);
            }
        };

        let maybe_row_count: Option<u32> = match tx.query_first(
            r"SELECT COUNT(*) as cnt
            FROM puppers@replica.puppers AS p"
        ) {
            Ok(v) => v,
            Err(e) => {
                self.pool.set_is_working(false);
                return Err(e);
            }
        };

        if let Some(row_count) = maybe_row_count {
            let mut rng = rand::thread_rng();
            let rand_row_num: u32 = rng.gen_range(0, row_count-1);

            let r: Option<(u64, String, String)> = match tx.query_first(
                format!(r"SELECT p.id, p.name, p.image
                FROM puppers@replica.puppers AS p
                LIMIT {},1", rand_row_num)
            ) {
                Ok(v) => v,
                Err(e) => {
                    self.pool.set_is_working(false);
                    return Err(e);
                }
            };

            drop(tx);

            if let Some(pup) = r {
                return self.pup_with_rating(pup);
            }
        }

        // Didn't find a pupper :-(
        Ok(None)
    }

    fn get_top_ten(&mut self) -> Result<Option<Vec<Pupper>>, mysql::Error> {
        let winners: Vec<(u64, f64)> =
            self.pool.get_conn().unwrap().query_iter(
                r"SELECT r.pupper_id as pupper_id, COALESCE(SUM(r.rating)/COUNT(r.rating),0.0) as rating
                FROM puppers@replica.ratings AS r
                GROUP BY r.pupper_id
                ORDER BY rating desc
                LIMIT 10"
            ).map(|result| {
                result.map(|row_result| {
                    row_result.map(|r| mysql::from_row(r))
                })
            })?.collect::<Result<Vec<(u64, f64)>, mysql::Error>>()?;

        let winning_pups = self.puppers_from_rating_list(winners)?;

        Ok(Some(winning_pups))
    }
}

// WRAPPERS

// PupperRepositoryRetryWrapper is a retry wrapper that is agnostic to underlying storage.
pub struct PupperRepositoryRetryWrapper<T>
    where T: PupperRepository
{
    repo: T
}

impl<T> PupperRepositoryRetryWrapper<T>
    where T: PupperRepository
{
    pub fn new(repo: T) -> PupperRepositoryRetryWrapper<T> {
        PupperRepositoryRetryWrapper {
            repo,
        }
    }
}

impl<T> PupperRepository for PupperRepositoryRetryWrapper<T>
    where T: PupperRepository
{
    type Error = T::Error;

    fn get(&mut self, pupper_id: u64) -> Result<Option<Pupper>, Self::Error> {
        for _ in 0..RETRY_COUNT {
            if let Ok(result) = self.repo.get(pupper_id) {
                return Ok(result);
            }
        }

        self.repo.get(pupper_id)
    }

    fn get_random(&mut self) -> Result<Option<Pupper>, Self::Error> {
        for _ in 0..RETRY_COUNT {
            if let Ok(result) = self.repo.get_random() {
                return Ok(result);
            }
        }

        self.repo.get_random()
    }

    fn get_top_ten(&mut self) -> Result<Option<Vec<Pupper>>, Self::Error> {
        for _ in 0..RETRY_COUNT {
            if let Ok(result) = self.repo.get_top_ten() {
                return Ok(result);
            }
        }

        self.repo.get_top_ten()
    }
}

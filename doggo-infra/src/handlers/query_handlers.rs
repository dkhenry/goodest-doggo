use mysql;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetPupperQuery, GetRandomPupperQuery, GetTopTenPuppersQuery};
use doggo_core::dtos::Pupper;
use super::CLIENT_POOL;
use super::Conn;
use std::collections::HashMap;

pub struct VitessPupperQueriesHandler {
    conn: Conn,
}

impl VitessPupperQueriesHandler {
    /// Associative function to create a new query handler from a connection.
    pub fn new() -> VitessPupperQueriesHandler {
        VitessPupperQueriesHandler {
            // "Clone" the pool (it's an Arc, so just increase count) and then get a connection for use
            // in this handler.
            conn: CLIENT_POOL.clone().get_conn().unwrap(),
        }
    }
    // May optionally return a rating upon successful db interaction.  Underlying db
    // error will be communicated as a mysql::Error in the Err result variant returned.
    fn puppers_rating(&mut self, name: &String) -> Result<Option<f64>, mysql::Error> {
        match self.conn.prep_exec(
            r"SELECT COALESCE(SUM(r.rating)/COUNT(r.rating),0.0)
            FROM puppers@replica.ratings as r
            WHERE r.pupper_name=?",
            (name,)
        ) {
            Ok(mut qr) => {
                if let Some(row_result) = qr.next() {
                    let row = row_result?;
                    return Ok(Some(mysql::from_row(row)));
                }
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    // TODO: If this returns an error, then we have underlying db failure and need to retry.
    fn pup_with_rating(
        &mut self,
        pup_data: (u64, String, String)
    ) -> Result<Option<Pupper>, mysql::Error> {
        let maybe_rating = self.puppers_rating(&pup_data.1)?;
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
        list: Vec<(String, f64)>,
    ) -> Result<Vec<Pupper>, mysql::Error> {
        let rating_mapping: HashMap<String, f64> = list.into_iter().collect();
        let names: Vec<&String> = rating_mapping.iter().map(|p| p.0).collect();

        // TODO: With this current design if we fail to get any of these puppers then we don't return
        // a pupper for that row.  That means we could potentially have a final Vec that has less
        // than 10 puppers in it due to lookup failure.  Figure out a better way to handle this.
        let puppers: Vec<Pupper> =
            self.conn.prep_exec(
                r"SELECT p._id as id, p.name as name, p.image as image
                FROM puppers AS p
                WHERE name in
                (?,?,?,?,?,?,?,?,?,?)",
                names
            ).map(|mut result| {
                // TODO: Fix this unwrap.
                result.map(|row_result| row_result.unwrap() ).map(|row| {
                    let (id, name, image) = mysql::from_row(row);
                    let rating = Some(*rating_mapping.get(&name).unwrap());
                    Pupper {
                        id,
                        name,
                        image,
                        rating,
                    }
                }).collect()
            })?;

        Ok(puppers)
    }
}

impl HandlesQuery<GetPupperQuery> for VitessPupperQueriesHandler {
    type Result = Result<Option<Pupper>, mysql::Error>;

    fn handle(&mut self, query: GetPupperQuery) -> Self::Result {
        let r: Option<(u64, String, String)> =
            match self.conn.prep_exec(
                r"SELECT p._id, p.name, p.image
                FROM puppers AS p
                WHERE name = ?",
                (query.name,)
            ) {
                Ok(mut qr) => {
                    if let Some(row_result) = qr.next() {
                        let row = row_result?;
                        Some(mysql::from_row::<(u64, String, String)>(row))
                    } else {
                        None
                    }
                },

                // Underlying MySQL error type unrelated to existence of puppers in db.
                Err(e) => {
                    return Err(e);
                }
            };

        if let Some(pup) = r {
            return self.pup_with_rating(pup);
        }

        // Didn't find a pupper :-(
        Ok(None)
    }
}

impl HandlesQuery<GetRandomPupperQuery> for VitessPupperQueriesHandler {
    type Result = Result<Option<Pupper>, mysql::Error>;

    #[allow(unused_variables)]
    fn handle(&mut self, query: GetRandomPupperQuery) -> Self::Result {
        let r: Option<(u64, String, String)> =
            match self.conn.query(
                r"SELECT p._id, p.name, p.image
                FROM puppers@replica.puppers AS p
                ORDER BY RAND()
                LIMIT 1"
            ) {
                Ok(mut qr) => {
                    if let Some(row_result) = qr.next() {
                        let row = row_result?;
                        Some(mysql::from_row::<(u64, String, String)>(row))
                    } else {
                        None
                    }
                },

                // Underlying MySQL error type unrelated to existence of puppers in db.
                Err(e) => {
                    return Err(e);
                }
            };

        if let Some(pup) = r {
            return self.pup_with_rating(pup);
        }

        // Didn't find a pupper :-(
        Ok(None)
    }
}

impl HandlesQuery<GetTopTenPuppersQuery> for VitessPupperQueriesHandler {
    type Result = Result<Option<Vec<Pupper>>, mysql::Error>;

    #[allow(unused_variables)]
    fn handle(&mut self, query: GetTopTenPuppersQuery) -> Self::Result {
        let winners: Vec<(String, f64)> =
        self.conn.query(
            r"SELECT r.pupper_name as name, COALESCE(SUM(r.rating)/COUNT(r.rating),0.0) as rating
                FROM puppers@replica.ratings AS r
                GROUP BY r.pupper_name
                ORDER BY rating asc
                LIMIT 10"
        ).map(|mut result| {
            // TODO: This might break with the nested unwrap.
            result.map(|row_result| row_result.unwrap() ).map(|row| {
                mysql::from_row(row)
            }).collect()
        })?;

        let winning_pups = self.puppers_from_rating_list(winners)?;

        Ok(Some(winning_pups))
    }
}

use mysql;
use domain_patterns::query::HandlesQuery;
use doggo_core::queries::pupper_queries::{GetPupperQuery, GetRandomPupperQuery, PupperQueries};
use doggo_core::dtos::Pupper;
use super::CLIENT_POOL;
use super::Conn;

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
            FROM doggers@replica.ratings as r
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
                FROM doggers@replica.puppers AS p
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

impl HandlesQuery<PupperQueries> for VitessPupperQueriesHandler {
    type Result = Result<Option<Pupper>, mysql::Error>;

    fn handle(&mut self, query: PupperQueries) -> Self::Result {
        match query {
            PupperQueries::GetRandomPupper(q) => self.handle(q),
            PupperQueries::GetPupper(q) => self.handle(q),
        }
    }
}

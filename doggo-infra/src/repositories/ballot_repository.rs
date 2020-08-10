use super::CLIENT_POOL;
use super::Pool;
use doggo_core::collection_abstractions::BallotRepository;
use doggo_core::ballot::Ballot;
use crate::errors::Error;

pub struct VitessBallotRepository {
    pool: Pool,
}

impl VitessBallotRepository {
    /// Associative function to create a new command handler from a connection.
    pub fn new() -> VitessBallotRepository {
        VitessBallotRepository {
            // "Clone" the pool (it's an Arc, so just increase count) and then get a connection for use
            // in this handler.
            pool: CLIENT_POOL.clone(),
        }
    }
}

impl BallotRepository for VitessBallotRepository {
    type Error = Error;

    fn insert(&mut self, ballot: &Ballot) -> Result<(), Self::Error> {
        match self.pool.query_drop(format!(r"INSERT INTO ratings (pupper_id, rating, user_id)
            VALUES ({}, {}, '{}')", &ballot.pupper_id, &ballot.rating, &ballot.user_id)
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e))
        }
    }
}

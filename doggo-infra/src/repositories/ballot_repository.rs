use super::CLIENT_POOL;
use super::Conn;
use doggo_core::collection_abstractions::BallotRepository;
use doggo_core::ballot::Ballot;
use crate::errors::Error;

pub struct VitessBallotRepository {
    conn: Conn,
}

impl VitessBallotRepository {
    /// Associative function to create a new command handler from a connection.
    pub fn new() -> VitessBallotRepository {
        VitessBallotRepository {
            // "Clone" the pool (it's an Arc, so just increase count) and then get a connection for use
            // in this handler.
            conn: CLIENT_POOL.clone().get_conn().unwrap(),
        }
    }
}

impl BallotRepository for VitessBallotRepository {
    type Error = Error;

    fn insert(&mut self, ballot: &Ballot) -> Result<(), Self::Error> {
        match self.conn.query(
            format!(r"INSERT INTO ratings (pupper_id, rating)
            VALUES ('{}','{}')", &ballot.pupper_id, &ballot.rating)
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e))
        }
    }
}

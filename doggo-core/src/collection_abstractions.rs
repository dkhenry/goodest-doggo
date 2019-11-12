use crate::dtos::Pupper;
use crate::ballot::Ballot;
use crate::user::User;

/// PupperRepository for now is only needed for query methods, because we insert puppers directly
/// into the database with a shell script.  In the future if we want to add the ability for users
/// to add their own puppers, we would add insert and mutation methods here.
pub trait PupperRepository {
    type Error: 'static + Send;

    fn get(&mut self, pupper_id: u64) -> Result<Option<Pupper>, Self::Error>;
    fn get_random(&mut self) -> Result<Option<Pupper>, Self::Error>;
    fn get_top_ten(&mut self) -> Result<Option<Vec<Pupper>>, Self::Error>;
}

/// BallotRepository represents a collection of ballots.  A ballot in this sense is a vote cast
/// for a pupper.  Right now we only use this for inserts.  Most queries for top puppers, or a puppers
/// rating are done on the PupperRepository because rating is fully hydrated for each pupper,
/// and for top ranked queries since we need to get the puppers for those anyways.
pub trait BallotRepository {
    type Error: 'static + Send;

    fn insert(&mut self, ballot: &Ballot) -> Result<(), Self::Error>;
}

pub trait UserRepository {
    type Error: std::error::Error + std::fmt::Display + 'static + Send;

    fn get(&mut self, email: &String) -> Result<Option<User>, Self::Error>;
    fn insert(&mut self, user: &User) -> Result<Option<String>, Self::Error>;
}
/// Represents a ballot cast for a pupper.
/// TODO: Add user_id once we add capability to have users.
pub struct Ballot {
    pub rating: u64,
    pub pupper_id: u64,
}
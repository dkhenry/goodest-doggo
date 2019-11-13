use ulid::Ulid;

/// Represents a ballot cast for a pupper.
pub struct Ballot {
    pub rating: u64,
    pub pupper_id: u64,
    pub user_id: Ulid,
}
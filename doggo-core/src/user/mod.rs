use domain_patterns::models::ValueObject;
use crate::value_objects::Username;

/// User represents a user in our system.  The password string is encrypted.
pub struct User {
    pub username: Username,
    pub password: String,
}

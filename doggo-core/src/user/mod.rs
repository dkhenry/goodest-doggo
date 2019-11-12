use domain_patterns::models::ValueObject;
use crate::Result;
use crate::value_objects::{Username, Password, Email};
use std::convert::TryFrom;

/// User represents a user in our system.  The password value object always stores an encrypted string.
pub struct User {
    email: Email,
    username: Username,
    password: Password,
}

impl User {
    pub fn new(email: String, username: String, password: String) -> Result<User> {
        Ok(User {
            email: Email::try_from(email)?,
            username: Username::try_from(username)?,
            password: Password::try_from(password)?,
        })
    }

    pub fn validate_password(&self, password: String) -> bool {
        self.password.validate_password(password)
    }
}

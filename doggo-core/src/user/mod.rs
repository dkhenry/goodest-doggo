use crate::Result;
use crate::value_objects::{Password, Email};
use std::convert::TryFrom;

/// User represents a user in our system.  The password value object always stores an encrypted string.
#[derive(Entity, Clone)]
pub struct User {
    id: i64,
    email: Email,
    password: Password,
}

impl User {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn new(email: String, password: String) -> Result<User> {
        Ok(User {
            id: rand::random(),
            email: Email::try_from(email)?,
            password: Password::try_from(password)?,
        })
    }

    /// This method allows you to construct a user bypassing validation from raw values.
    pub fn new_raw(id: i64, email: String, password: String) -> User {
        User {
            id: id,
            email: Email { value: email },
            password: Password { value: password },
        }
    }

    pub fn valid_password(&self, password: &String) -> bool {
        self.password.matches(password)
    }
}

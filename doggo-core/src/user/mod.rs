use crate::Result;
use crate::value_objects::{Password, Email};
use std::convert::TryFrom;
use ulid::Ulid;

/// User represents a user in our system.  The password value object always stores an encrypted string.
#[derive(Entity, Clone)]
pub struct User {
    id: Ulid,
    email: Email,
    password: Password,
}

impl User {
    pub fn new(email: String, password: String) -> Result<User> {
        Ok(User {
            id: Ulid::new(),
            email: Email::try_from(email)?,
            password: Password::try_from(password)?,
        })
    }

    /// Returns the underlying u128 inside the ulid for storing as a binary type.
    pub fn bin_id(&self) -> u128 {
        self.id.0
    }

    /// Returns the underlying id as an owned Ulid.
    pub fn raw_id(&self) -> Ulid {
        self.id.clone()
    }

    /// This method allows you to construct a user bypassing validation from raw values.
    pub fn new_raw(id: String, email: String, password: String) -> User {
        User {
            id: Ulid::from_string(&id).unwrap(),
            email: Email { value: email },
            password: Password { value: password },
        }
    }

    pub fn valid_password(&self, password: &String) -> bool {
        self.password.matches(password)
    }
}

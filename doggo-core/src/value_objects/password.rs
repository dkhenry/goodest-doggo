use crate::errors::{Error, Result};
use crate::value_objects::ValidationError;
use domain_patterns::models::ValueObject;

#[derive(Debug, PartialEq, Clone)]
pub struct Password {
    pub value: String,
}

impl Password {
    fn hash_password(password: String) -> String {
        // TODO: Gracefully handle error.
        bcrypt::hash(password, 10).unwrap()
    }

    // TODO: Change to return error type and act as a guard. Useful for changing account details.
    pub fn matches(&self, password: &String) -> bool {
        bcrypt::verify(password, &self.value).unwrap()
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::convert::TryFrom<String> for Password {
    type Error = crate::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::validate(&value)?;

        Ok(Password {
            value: Self::hash_password(value),
        })
    }
}

impl ValueObject<String> for Password {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        const MIN_LEN: usize = 8;

        if value.len() < MIN_LEN {
            return Err(
                ValidationError::PasswordValidationError {
                    msg: format!("length must be greater than {}", MIN_LEN),
                }.into()
            );
        };

        Ok(())
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
}

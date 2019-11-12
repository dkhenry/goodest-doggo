use crate::errors::{Error, Result};
use crate::value_objects::ValidationError;
use domain_patterns::models::ValueObject;
use regex::Regex;

#[derive(ValueSetup, Debug)]
pub struct Username {
    pub value: String,
}

impl ValueObject<String> for Username {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        const MIN_LEN: usize = 4;
        const MAX_LEN: usize = 64;
        let username_rx = Regex::new(r#"^[A-Za-z0-9\\_\\.]+$"#).unwrap();

        if value.len() < MIN_LEN || value.len() > MAX_LEN {
            return Err(
                ValidationError::UsernameValidationError {
                    msg: format!("length must be between {} and {}", MIN_LEN, MAX_LEN),
                }.into()
            );
        };

        if !username_rx.is_match(value) {
            return Err(
                ValidationError::UsernameValidationError {
                    msg: format!("must contain only alphanumeric characters, underscores or periods"),
                }.into()
            );
        }

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

    #[test]
    fn can_create_valid_username() -> Result<()> {
        let username = Username::try_from("User1234".to_string())?;
        Ok(())
    }

    #[test]
    fn cant_create_short_username() {
        let result = Username::try_from("Use".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn cant_create_long_username() {
        let long_username = std::str::from_utf8(&[9; 65]).unwrap().to_string();
        let result = Username::try_from(long_username);
        assert!(result.is_err());
    }

    #[test]
    fn cant_create_invalid_char_username() {
        let invalid_chars = std::str::from_utf8(&[0; 32]).unwrap().to_string();
        let result = Username::try_from(invalid_chars);
        assert!(result.is_err());
    }
}
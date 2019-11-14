use crate::errors::{Error, Result};
use crate::value_objects::ValidationError;
use domain_patterns::models::ValueObject;
use regex::Regex;

#[derive(ValueSetup)]
pub struct Email {
    pub value: String,
}

impl ValueObject<String> for Email {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        let email_rx = Regex::new(
            r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
        ).unwrap();

        if !email_rx.is_match(value) {
            return Err(ValidationError::EmailValidationError.into());
        }

        Ok(())
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}
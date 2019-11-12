use snafu::Snafu;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("{}", source))]
    ValidationError {
        source: crate::value_objects::ValidationError,
    },
}

impl From<crate::value_objects::ValidationError> for Error {
    fn from(err: crate::value_objects::ValidationError) -> Self {
        Error::ValidationError {
            source: err,
        }
    }
}
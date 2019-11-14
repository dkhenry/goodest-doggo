use snafu::Snafu;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{}", source))]
    ValidationError {
        source: crate::value_objects::ValidationError,
    },

    #[snafu(display("resource '{}' was not found", resource))]
    ResourceNotFound {
        resource: String,
    },

    /// NotAuthorized conveys that the caller is not authorized to commit the action.
    #[snafu(display("not authorized"))]
    NotAuthorized,

    #[snafu(display("{}", source))]
    DbFailure {
        source: Box<dyn std::error::Error + Send>,
    },
}

impl From<crate::value_objects::ValidationError> for Error {
    fn from(err: crate::value_objects::ValidationError) -> Self {
        Error::ValidationError {
            source: err,
        }
    }
}
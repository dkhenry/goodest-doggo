use snafu::Snafu;
use std::result;

// TODO: Should we even be dealing with these directly here?
use mysql;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    /// UniqueViolation conveys some kind of a unique key constraint being violated, either by duplicate
    /// entry, or trying to use a key that is already in use.
    #[snafu(display("Unique key violation."))]
    UniqueViolation,

    /// UnknownDbFailure conveys to the caller that some kind of error happened when communicating with
    /// an underlying database that we have not setup error translation for yet.
    #[snafu(display("{}", source))]
    UnknownDbFailure {
        source: Box<dyn std::error::Error + Send>,
    },
}

impl From<mysql::Error> for Error {
    fn from(err: mysql::Error) -> Self {
        if let mysql::Error::MySqlError(e) = &err {
            if e.code == mysql::ServerError::ER_DUP_ENTRY as u16 {
                return Error::UniqueViolation
            }
            // TODO: Add ways to deal with other errors as we actually encounter them.
        }

        // TODO: Add ways to deal with other errors as we actually encounter them.
        return Error::UnknownDbFailure { source: Box::new(err) }
    }
}

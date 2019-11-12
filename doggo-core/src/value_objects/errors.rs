use snafu::Snafu;

/// ValidationErrors are errors related to failure to validate during creation of a value object.
#[derive(Debug, Snafu, PartialEq)]
pub enum ValidationError {
    #[snafu(display("Username supplied is invalid: {}", msg))]
    UsernameValidationError {
        msg: String,
    },
}
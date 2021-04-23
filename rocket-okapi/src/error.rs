use std::error::Error;
use std::fmt;

/// Type alias for `Result<T, OpenApiError>`.
pub type Result<T> = std::result::Result<T, OpenApiError>;

/// The error type returned by `rocket_okapi` when something fails.
#[derive(Debug, Clone)]
pub struct OpenApiError {
    msg: String,
}

impl OpenApiError {
    /// Create a new `OpenApiError` containing a message.
    #[must_use] pub fn new(msg: String) -> Self {
        OpenApiError { msg }
    }
}

impl fmt::Display for OpenApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for OpenApiError {}

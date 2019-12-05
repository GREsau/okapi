use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, OpenApiError>;

#[derive(Debug, Clone)]
pub struct OpenApiError {
    msg: String,
}

impl OpenApiError {
    pub fn new(msg: String) -> Self {
        OpenApiError { msg }
    }
}

impl fmt::Display for OpenApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for OpenApiError {}

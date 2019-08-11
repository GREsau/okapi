mod error;
mod responses_impls;

pub mod gen;
pub mod handler;
pub mod util;

pub use error::*;

pub struct OperationInfo {
    pub path: String,
    pub method: rocket::http::Method,
    pub operation: okapi::openapi3::Operation,
}

pub trait OpenApiResponses {
    fn responses(gen: &mut gen::OpenApiGenerator) -> Result;
}

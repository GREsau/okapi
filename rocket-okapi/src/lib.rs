#![feature(specialization)]

mod error;
mod responder_impls;

pub mod gen;
pub mod handler;
pub mod settings;
pub mod util;

pub use error::*;
pub use rocket_okapi_codegen::*;

pub struct OperationInfo {
    pub path: String,
    pub method: rocket::http::Method,
    pub operation: okapi::openapi3::Operation,
}

pub trait OpenApiResponder<'r>: rocket::response::Responder<'r> {
    fn responses(gen: &mut gen::OpenApiGenerator) -> Result;
}

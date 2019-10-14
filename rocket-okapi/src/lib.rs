#![feature(specialization)]

mod error;

pub mod gen;
pub mod handlers;
pub mod request;
pub mod response;
pub mod settings;
pub mod swagger_ui;
pub mod util;

pub use error::*;
pub use rocket_okapi_codegen::*;
pub use schemars::JsonSchema;

pub struct OperationInfo {
    pub path: String,
    pub method: rocket::http::Method,
    pub operation: okapi::openapi3::Operation,
}

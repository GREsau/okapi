mod from_data_impls;
mod from_param_impls;

use super::gen::OpenApiGenerator;
use super::Result;
use okapi::openapi3::*;

pub trait OpenApiFromData<'r>: rocket::data::FromData<'r> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result<RequestBody>;
}

pub trait OpenApiFromParam<'r>: rocket::request::FromParam<'r> {
    fn path_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

pub trait OpenApiFromSegments<'r>: rocket::request::FromSegments<'r> {
    fn path_multi_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

pub trait OpenApiFromFormValue<'r>: rocket::request::FromFormValue<'r> {
    fn query_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

pub trait OpenApiFromQuery<'r>: rocket::request::FromQuery<'r> {
    fn query_multi_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

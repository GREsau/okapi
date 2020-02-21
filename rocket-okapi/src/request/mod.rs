mod from_data_impls;
mod from_param_impls;

use super::gen::OpenApiGenerator;
use super::Result;
use okapi::openapi3::*;

/// This trait means that the implementer can be used as a `FromData` request guard, and that this
/// can also be documented.
pub trait OpenApiFromData<'r>: rocket::data::FromData<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromData` for
    /// implementer.
    fn request_body(gen: &mut OpenApiGenerator) -> Result<RequestBody>;
}

/// This trait means that the implementer can be used as a `FromParam` request guard, and that this
/// can also be documented.
pub trait OpenApiFromParam<'r>: rocket::request::FromParam<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromParam` for
    /// implementer.
    fn path_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

/// This trait means that the implementer can be used as a `FromSegments` request guard, and that
/// this can also be documented.
pub trait OpenApiFromSegments<'r>: rocket::request::FromSegments<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromSegments`
    /// for implementer.
    fn path_multi_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

/// This trait means that the implementer can be used as a `FromFormValue` request guard, and that
/// this can also be documented.
pub trait OpenApiFromFormValue<'r>: rocket::request::FromFormValue<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromFormValue`
    /// for implementer.
    fn query_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

/// This trait means that the implementer can be used as a `FromQuery` request guard, and that this
/// can also be documented.
pub trait OpenApiFromQuery<'r>: rocket::request::FromQuery<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromQuery` for
    /// implementer.
    fn query_multi_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

mod from_data_impls;
mod from_form_multi_param_impls;
mod from_form_param_impls;
mod from_param_impls;
mod from_segments_impls;

use super::gen::OpenApiGenerator;
use super::Result;
use okapi::openapi3::{Parameter, RequestBody};

/// Expose this to the public to be use when manually implementing a
/// [Form Guard](https://api.rocket.rs/master/rocket/form/trait.FromForm.html).
pub use from_form_multi_param_impls::get_nested_form_parameters;

/// This trait means that the implementer can be used as a `FromData` request guard,
/// and that this can also be documented.
pub trait OpenApiFromData<'r>: rocket::data::FromData<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromData` for
    /// implementer.
    fn request_body(gen: &mut OpenApiGenerator) -> Result<RequestBody>;
}

/// This trait means that the implementer can be used as a `FromParam` request guard,
/// and that this can also be documented.
pub trait OpenApiFromParam<'r>: rocket::request::FromParam<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromParam` for
    /// implementer. Path parameters.
    fn path_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

/// This trait means that the implementer can be used as a `FromSegments` request guard,
/// and that this can also be documented.
pub trait OpenApiFromSegments<'r>: rocket::request::FromSegments<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromSegments`
    /// for implementer.
    fn path_multi_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

/// This trait means that the implementer can be used as a `FromFormField` request guard,
/// and that this can also be documented.
pub trait OpenApiFromFormField<'r>: rocket::form::FromFormField<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromFormField`
    /// for implementer.
    fn form_parameter(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> Result<Parameter>;
}

/// This trait means that the implementer can be used as a `FromForm` request guard,
/// and that this can also be documented.
pub trait OpenApiFromForm<'r>: rocket::form::FromForm<'r> {
    /// Return a `RequestBody` containing the information required to document the `FromForm` for
    /// implementer.
    fn form_multi_parameter(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> Result<Vec<Parameter>>;
}

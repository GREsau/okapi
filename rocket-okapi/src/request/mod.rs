mod from_data_impls;
mod from_form_multi_param_impls;
mod from_form_param_impls;
mod from_param_impls;
mod from_request_impls;
mod from_segments_impls;

use super::gen::OpenApiGenerator;
use super::Result;
use okapi::openapi3::{Parameter, RequestBody, Responses, SecurityRequirement, SecurityScheme};

/// Expose this to the public to be use when manually implementing a
/// [Form Guard](https://api.rocket.rs/master/rocket/form/trait.FromForm.html).
pub use from_form_multi_param_impls::get_nested_form_parameters;

/// This trait is used to document the request body that implements
/// [`FromData`](rocket::data::FromData).
pub trait OpenApiFromData<'r>: rocket::data::FromData<'r> {
    /// Return a [`RequestBody`] containing the information required to document the
    /// [`FromData`](rocket::data::FromData) object.
    fn request_body(gen: &mut OpenApiGenerator) -> Result<RequestBody>;
}

/// This trait is used to document a dynamic part of a path that implements
/// [`FromParam`](rocket::request::FromParam).
/// For example `<user_id>` in route path.
pub trait OpenApiFromParam<'r>: rocket::request::FromParam<'r> {
    /// Return a [`Parameter`] containing the information required to document the
    /// [`FromParam`](rocket::request::FromParam) path parameter.
    fn path_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

/// This trait is used to document a dynamic path segment that implements
/// [`FromSegments`](rocket::request::FromSegments).
/// For example `<param..>` in route path.
pub trait OpenApiFromSegments<'r>: rocket::request::FromSegments<'r> {
    /// Return a [`Parameter`] containing the information required to document the
    /// [`FromSegments`](rocket::request::FromSegments) path parameter.
    fn path_multi_parameter(gen: &mut OpenApiGenerator, name: String) -> Result<Parameter>;
}

/// This trait is used to document a query guard segment that implements
/// [`FromFormField`](rocket::form::FromFormField).
/// For example `?<param>` in the route's query part.
pub trait OpenApiFromFormField<'r>: rocket::form::FromFormField<'r> {
    /// Return a [`Parameter`] containing the information required to document the
    /// [`FromFormField`](rocket::form::FromFormField) route's query part.
    fn form_parameter(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> Result<Parameter>;
}

/// This trait is used to document multiple query guard segments that implement
/// [`FromForm`](rocket::form::FromForm).
/// For example `?<param>` in the route's query part.
pub trait OpenApiFromForm<'r>: rocket::form::FromForm<'r> {
    /// Return a [`Vec<Parameter>`] containing the information required to document the
    /// [`FromForm`](rocket::form::FromForm) route's query part.
    fn form_multi_parameter(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> Result<Vec<Parameter>>;
}

/// Used as a return type for [`OpenApiFromRequest`] trait.
/// Defines what requirements a Request Guard needs in order to be validated.
#[allow(clippy::large_enum_variant)]
pub enum RequestHeaderInput {
    /// This request header requires no input anywhere
    None,
    /// Useful for when you want to set a header per route.
    Parameter(Parameter),
    /// The request guard implements a security scheme.
    ///
    /// Parameters:
    /// - The name of the [`SecurityScheme`].
    /// - [`SecurityScheme`] is global definition of the authentication (per OpenApi spec).
    /// - [`SecurityRequirement`] is the requirements for the route.
    Security(String, SecurityScheme, SecurityRequirement),
    /// A server this resources is allocated on.
    ///
    /// Parameters:
    /// - The url
    /// - The description
    /// - Variable mapping: A map between a variable name and its value.
    /// The value is used for substitution in the server’s URL template.
    Server(
        String,
        Option<String>,
        okapi::Map<String, okapi::openapi3::ServerVariable>,
    ),
}

// Re-export derive trait here for convenience.
pub use rocket_okapi_codegen::OpenApiFromRequest;

/// Trait that needs to be implemented for all types that implement
/// [`FromRequest`](rocket::request::FromRequest).
/// This trait specifies what headers or other parameters are required for this
/// [Request Guards](https://rocket.rs/v0.5/guide/requests/#request-guards)
/// to be validated successfully.
///
/// If it does not quire any headers or parameters you can use the derive macro:
/// ```rust,ignore
/// use rocket_okapi::request::OpenApiFromRequest;
///
/// #[derive(OpenApiFromRequest)]
/// pub struct MyStructName;
/// ```
pub trait OpenApiFromRequest<'a>: rocket::request::FromRequest<'a> {
    /// Specifies what headers or other parameters are required for this Request Guards to validate
    /// successfully.
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> Result<RequestHeaderInput>;

    /// Optionally add responses to the Request Guard.
    /// This can be used for when the request guard could return a "401 Unauthorized".
    /// Or any other responses, other then one from the default response.
    fn get_responses(_gen: &mut OpenApiGenerator) -> Result<Responses> {
        Ok(Responses::default())
    }
}

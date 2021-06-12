mod responder_impls;

use super::gen::OpenApiGenerator;
use super::Result;
use okapi::openapi3::Responses;

/// See `OpenApiResponderInner`. This is a wrapper around
/// `OpenApiResponderInner` that ensures the implementor is a
/// `rocket::response::Responder`.
pub trait OpenApiResponder<'a, 'r: 'a>: rocket::response::Responder<'a, 'r> {
    /// Create the responses type, which is a list of responses that can be
    /// rendered in `openapi.json` format.
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses>;
}

impl<'a, 'r: 'a, T: OpenApiResponderInner> OpenApiResponder<'a, 'r> for T
where
    T: rocket::response::Responder<'a, 'r>,
{
    #[inline(always)]
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses> {
        T::responses(gen)
    }
}

/// Implementing this trait means that any route returning the implementer can
/// be marked with `#[openapi]`, and that the route can be documented.
pub trait OpenApiResponderInner {
    /// Create the responses type, which is a list of responses that can be
    /// rendered in `openapi.json` format.
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses>;
}

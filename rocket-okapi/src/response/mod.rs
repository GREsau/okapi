mod responder_impls;

use super::gen::OpenApiGenerator;
use super::Result;
use okapi::openapi3::Responses;

/// Implementing this trait means that any route returning the implementer can me marked with
/// `#[openapi]`, and that the route can be documented.
pub trait OpenApiResponder<'r>: rocket::response::Responder<'r> {
    /// Create the responses type, which is a list of responses that can be rendered in
    /// `openapi.json` format.
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses>;
}

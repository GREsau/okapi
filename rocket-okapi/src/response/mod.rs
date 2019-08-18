mod responder_impls;
use super::gen::OpenApiGenerator;
use super::Result;
use okapi::openapi3::Responses;

pub trait OpenApiResponder<'r>: rocket::response::Responder<'r> {
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses>;
}

use crate::{gen::OpenApiGenerator, OpenApiResponder};
use crate::{util::*, Result};
use rocket_contrib::json::{Json, JsonValue}; // TODO json feature flag
use schemars::{schema::SchemaObject, JsonSchema};
use serde::Serialize;

impl<T: JsonSchema + Serialize> OpenApiResponder<'_> for Json<T> {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Default::default();
        let schema = gen.json_schema::<T>()?;
        add_schema_response(&mut responses, 200, "application/json", schema)?;
        Ok(responses)
    }
}

impl OpenApiResponder<'_> for JsonValue {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Default::default();
        let schema = SchemaObject::default();
        add_schema_response(&mut responses, 200, "application/json", schema.into())?;
        Ok(responses)
    }
}

impl OpenApiResponder<'_> for () {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Default::default();
        add_response(&mut responses, 200);
        Ok(responses)
    }
}

impl<'r, T: OpenApiResponder<'r>> OpenApiResponder<'r> for Option<T> {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = T::responses(gen)?;
        add_response(&mut responses, 404);
        Ok(responses)
    }
}

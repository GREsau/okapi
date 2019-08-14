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

impl OpenApiResponder<'_> for String {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Default::default();
        let schema = gen.json_schema::<String>()?;
        add_schema_response(&mut responses, 200, "text/plain", schema)?;
        Ok(responses)
    }
}

impl<'r> OpenApiResponder<'r> for &'r str {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <String>::responses(gen)
    }
}

impl OpenApiResponder<'_> for Vec<u8> {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Default::default();
        add_content_response(
            &mut responses,
            200,
            "application/octet-stream",
            Default::default(),
        )?;
        Ok(responses)
    }
}

impl<'r> OpenApiResponder<'r> for &'r [u8] {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <Vec<u8>>::responses(gen)
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

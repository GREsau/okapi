use crate::{gen::OpenApiGenerator, OpenApiResponder};
use crate::{util, Result};
use rocket_contrib::json::{Json, JsonValue}; // TODO json feature flag
use schemars::JsonSchema;
use serde::Serialize;

impl<T: JsonSchema + Serialize> OpenApiResponder<'_> for Json<T> {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        Ok(util::schema_response(
            200,
            "application/json",
            gen.json_schema::<T>()?,
        ))
    }
}

impl OpenApiResponder<'_> for JsonValue {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let schema_gen = gen.schema_generator();
        let schema = schema_gen.get_schema_object(&gen.schema_generator().schema_for_any())?;
        Ok(util::schema_response(
            200,
            "application/json",
            schema.into(),
        ))
    }
}

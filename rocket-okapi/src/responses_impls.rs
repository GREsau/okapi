use crate::{gen::OpenApiGenerator, OpenApiResponses};
use crate::{util, Result};
use rocket_contrib::json::{Json, JsonValue}; // TODO json feature flag
use schemars::JsonSchema;

impl<T: JsonSchema> OpenApiResponses for Json<T> {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        Ok(util::simple_responses(
            200,
            "application/json",
            gen.json_schema::<T>()?,
        ))
    }
}

impl OpenApiResponses for JsonValue {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let schema = gen.get_ref_or_object(gen.schema_generator().schema_for_any())?;
        Ok(util::simple_responses(200, "application/json", schema))
    }
}

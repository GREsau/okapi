use super::OpenApiFromData;
use crate::gen::OpenApiGenerator;
use okapi::{openapi3::*, Map};
use rocket_contrib::json::Json; // TODO json feature flag
use serde::Deserialize;
type Result = crate::Result<RequestBody>;
use schemars::JsonSchema;

impl<'a, T: JsonSchema + Deserialize<'a>> OpenApiFromData<'a> for Json<T> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        let schema = gen.json_schema::<T>()?;
        Ok(RequestBody {
            content: {
                let mut map = Map::new();
                map.insert(
                    "application/json".to_owned(),
                    MediaType {
                        schema: Some(schema),
                        ..Default::default()
                    },
                );
                map
            },
            required: true,
            ..Default::default()
        })
    }
}

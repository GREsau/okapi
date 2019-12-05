use super::OpenApiFromData;
use crate::gen::OpenApiGenerator;
use okapi::{openapi3::*, Map};
use rocket_contrib::json::Json; // TODO json feature flag
use schemars::JsonSchema;
use serde::Deserialize;
use std::result::Result as StdResult;

type Result = crate::Result<RequestBody>;

impl<'a, T: JsonSchema + Deserialize<'a>> OpenApiFromData<'a> for Json<T> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        let schema = gen.json_schema::<T>();
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

impl<'a, T: OpenApiFromData<'a> + 'a> OpenApiFromData<'a> for StdResult<T, T::Error> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        T::request_body(gen)
    }
}

impl<'a, T: OpenApiFromData<'a> + 'a> OpenApiFromData<'a> for Option<T> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result {
        Ok(RequestBody {
            required: false,
            ..T::request_body(gen)?
        })
    }
}

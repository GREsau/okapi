//! Implement `OpenApiFromSegments` for everything that implements `FromSegments`
//! <https://docs.rs/rocket/0.5.1/rocket/request/trait.FromSegments.html#foreign-impls>

use super::OpenApiFromSegments;
use crate::gen::OpenApiGenerator;
use okapi::openapi3::{Object, Parameter, ParameterValue};
use schemars::JsonSchema;

type Result = crate::Result<Parameter>;

impl<'r, T> OpenApiFromSegments<'r> for T
where
    T: rocket::request::FromSegments<'r> + JsonSchema,
{
    fn path_multi_parameter(gen: &mut OpenApiGenerator, name: String) -> Result {
        let schema = gen.json_schema::<T>();
        Ok(Parameter {
            name,
            location: "path".to_owned(),
            description: None,
            required: true,
            deprecated: false,
            allow_empty_value: false,
            value: ParameterValue::Schema {
                style: None,
                explode: None,
                allow_reserved: true,
                schema,
                example: None,
                examples: None,
            },
            extensions: Object::default(),
        })
    }
}

use super::OpenApiFromParam;
use crate::gen::OpenApiGenerator;
use okapi::openapi3::{Object, Parameter, ParameterValue};
use schemars::JsonSchema;

type Result = crate::Result<Parameter>;

impl<'r, T> OpenApiFromParam<'r> for T
where
    T: rocket::request::FromParam<'r> + JsonSchema,
{
    fn path_parameter(gen: &mut OpenApiGenerator, name: String) -> Result {
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
                allow_reserved: false,
                schema,
                example: None,
                examples: None,
            },
            extensions: Object::default(),
        })
    }
}

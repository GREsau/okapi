//! Implement `OpenApiFromForm` for everything that implements `FromForm`
//! <https://docs.rs/rocket/0.5.1/rocket/form/trait.FromForm.html#foreign-impls>

use super::{get_nested_form_parameters, OpenApiFromForm, OpenApiFromFormField};
use crate::gen::OpenApiGenerator;
use okapi::openapi3::{Object, Parameter, ParameterValue};
use schemars::JsonSchema;

type Result = crate::Result<Parameter>;

impl<'r, T> OpenApiFromForm<'r> for T
where
    T: rocket::form::FromForm<'r> + JsonSchema,
{
    fn form_multi_parameter(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> crate::Result<Vec<Parameter>> {
        Ok(get_nested_form_parameters::<T>(gen, name, required))
    }
}

impl<'r, T> OpenApiFromFormField<'r> for T
where
    T: rocket::form::FromFormField<'r> + JsonSchema,
{
    fn form_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        let schema = gen.json_schema::<T>();
        Ok(Parameter {
            name,
            location: "query".to_owned(),
            description: None,
            required,
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

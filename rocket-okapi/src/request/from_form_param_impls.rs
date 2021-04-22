use super::{get_nested_form_parameters, OpenApiFromForm, OpenApiFromFormField};
use crate::gen::OpenApiGenerator;
use okapi::openapi3::{Parameter, ParameterValue};
use rocket::form::Result as FormResult;
use schemars::JsonSchema;

type Result = crate::Result<Parameter>;

macro_rules! impl_from_form_param {
    ($ty: path) => {
        impl<'r> OpenApiFromFormField<'r> for $ty {
            fn form_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
                let schema = gen.json_schema::<$ty>();
                Ok(Parameter {
                    name,
                    // Currently it's used also by query parameters which are more common than forms
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
                    extensions: okapi::Map::default(),
                })
            }
        }
    };
}

impl_from_form_param!(f32);
impl_from_form_param!(f64);
impl_from_form_param!(isize);
impl_from_form_param!(i8);
impl_from_form_param!(i16);
impl_from_form_param!(i32);
impl_from_form_param!(i64);
impl_from_form_param!(i128);
impl_from_form_param!(usize);
impl_from_form_param!(u8);
impl_from_form_param!(u16);
impl_from_form_param!(u32);
impl_from_form_param!(u64);
impl_from_form_param!(u128);
impl_from_form_param!(bool);
impl_from_form_param!(String);

impl<'r> OpenApiFromFormField<'r> for &'r str {
    fn form_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        let schema = gen.json_schema::<str>();
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
            extensions: okapi::Map::default(),
        })
    }
}

#[cfg(feature = "uuid")]
impl_from_form_param!(rocket::serde::uuid::Uuid);

// OpenAPI specification does not support optional path params, so we leave `required` as true,
// even for Options and Results.
impl<'r, T: OpenApiFromFormField<'r>> OpenApiFromFormField<'r> for FormResult<'r, T> {
    fn form_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        T::form_parameter(gen, name, required)
    }
}

impl<'r, T: OpenApiFromFormField<'r>> OpenApiFromFormField<'r> for Option<T> {
    fn form_parameter(gen: &mut OpenApiGenerator, name: String, _required: bool) -> Result {
        T::form_parameter(gen, name, false)
    }
}

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

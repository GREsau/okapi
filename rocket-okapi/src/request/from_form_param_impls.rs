use super::{OpenApiFromFormField, get_nested_form_parameters};
use crate::gen::OpenApiGenerator;
use okapi::openapi3::*;

type Result = crate::Result<Parameter>;

macro_rules! impl_from_query_param {
    ($ty: path) => {
        impl<'r> OpenApiFromFormField<'r> for $ty {
            fn form_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
                let schema = gen.json_schema::<$ty>();
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
                    extensions: Default::default(),
                })
            }
        }
    };
}

impl_from_query_param!(f32);
impl_from_query_param!(f64);
impl_from_query_param!(isize);
impl_from_query_param!(i8);
impl_from_query_param!(i16);
impl_from_query_param!(i32);
impl_from_query_param!(i64);
impl_from_query_param!(i128);
impl_from_query_param!(usize);
impl_from_query_param!(u8);
impl_from_query_param!(u16);
impl_from_query_param!(u32);
impl_from_query_param!(u64);
impl_from_query_param!(u128);
impl_from_query_param!(bool);
impl_from_query_param!(String);

// OpenAPI specification does not support optional path params, so we leave `required` as true,
// even for Options and Results.
impl<'r, T> OpenApiFromFormField<'r> for rocket::form::Result<'r, T>
where
    T: OpenApiFromFormField<'r>
{
    fn form_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        T::form_parameter(gen, name, required)
    }
}

impl<'r, T: OpenApiFromFormField<'r>> OpenApiFromFormField<'r> for Option<T> {
    fn form_parameter(gen: &mut OpenApiGenerator, name: String, _required: bool) -> Result {
        T::form_parameter(gen, name, false)
    }
}

impl<'r, T> super::OpenApiFromForm<'r> for T
where
    T: rocket::form::FromForm<'r> + schemars::JsonSchema,
{
    fn form_multi_parameter(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> crate::Result<Vec<Parameter>> {
        Ok(get_nested_form_parameters::<T>(gen, name, required))
    }
}
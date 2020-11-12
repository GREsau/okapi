use super::OpenApiFromFormValue;
use crate::gen::OpenApiGenerator;
use okapi::openapi3::*;
use std::result::Result as StdResult;
use rocket::http::RawStr;

type Result = crate::Result<Parameter>;

macro_rules! impl_from_query_param {
    ($ty: path) => {
        impl<'r> OpenApiFromFormValue<'r> for $ty {
            fn query_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
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
// Cow<> does not implement FromFormValue
// impl_from_query_param!(std::borrow::Cow<'r, str>);

// OpenAPI specification does not support optional path params, so we leave `required` as true,
// even for Options and Results.
impl<'r, T: OpenApiFromFormValue<'r>> OpenApiFromFormValue<'r> for StdResult<T, T::Error> {
    fn query_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        T::query_parameter(gen, name, required)
    }
}

impl<'r, T: OpenApiFromFormValue<'r>> OpenApiFromFormValue<'r> for Option<T> {
    fn query_parameter(gen: &mut OpenApiGenerator, name: String, _required: bool) -> Result {
        T::query_parameter(gen, name, false)
    }
}

impl<'r> OpenApiFromFormValue<'r> for &'r RawStr {
    fn query_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        let schema = gen.json_schema::<&str>();
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

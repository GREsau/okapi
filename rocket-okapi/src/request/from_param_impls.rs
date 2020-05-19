use super::OpenApiFromParam;
use crate::gen::OpenApiGenerator;
use okapi::openapi3::*;
use std::result::Result as StdResult;

type Result = crate::Result<Parameter>;

macro_rules! impl_from_param {
    ($ty: path) => {
        impl<'r> OpenApiFromParam<'r> for $ty {
            fn path_parameter(gen: &mut OpenApiGenerator, name: String) -> Result {
                let schema = gen.json_schema::<$ty>();
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
                    extensions: Default::default(),
                })
            }
        }
    };
}

impl_from_param!(f32);
impl_from_param!(f64);
impl_from_param!(isize);
impl_from_param!(i8);
impl_from_param!(i16);
impl_from_param!(i32);
impl_from_param!(i64);
impl_from_param!(i128);
impl_from_param!(usize);
impl_from_param!(u8);
impl_from_param!(u16);
impl_from_param!(u32);
impl_from_param!(u64);
impl_from_param!(u128);
impl_from_param!(bool);
impl_from_param!(String);
impl_from_param!(std::borrow::Cow<'r, str>);

// OpenAPI specification does not support optional path params, so we leave `required` as true,
// even for Options and Results.
impl<'r, T: OpenApiFromParam<'r>> OpenApiFromParam<'r> for StdResult<T, T::Error> {
    fn path_parameter(gen: &mut OpenApiGenerator, name: String) -> Result {
        T::path_parameter(gen, name)
    }
}

impl<'r, T: OpenApiFromParam<'r>> OpenApiFromParam<'r> for Option<T> {
    fn path_parameter(gen: &mut OpenApiGenerator, name: String) -> Result {
        T::path_parameter(gen, name)
    }
}

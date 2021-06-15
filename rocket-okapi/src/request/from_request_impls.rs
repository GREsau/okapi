use super::{OpenApiFromRequest, RequestHeaderInput};
use crate::gen::OpenApiGenerator;
use okapi::openapi3::*;

impl<'a, T: Send + Sync> OpenApiFromRequest<'a> for &'a rocket::State<T> {
    fn request_input(
        _gen: &mut OpenApiGenerator,
        name: String,
    ) -> crate::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::Parameter(Parameter {
            required: false,
            name: name,
            location: "header".into(),
            description: None,
            deprecated: false,
            allow_empty_value: true,
            extensions: Object::default(),
            value: ParameterValue::Content {
                content: Default::default(),
            },
        }))
    }
}

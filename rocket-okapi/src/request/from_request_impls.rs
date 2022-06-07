use super::{OpenApiFromRequest, RequestHeaderInput};
use crate::gen::OpenApiGenerator;
use okapi::openapi3::*;
use std::result::Result as StdResult;

// Implement `OpenApiFromRequest` for everything that implements `FromRequest`
// Order is same as on:
// https://docs.rs/rocket/0.5.0-rc.2/rocket/request/trait.FromRequest.html#foreign-impls

type Result = crate::Result<RequestHeaderInput>;

// ## Implementations on Foreign Types

impl<'r> OpenApiFromRequest<'r> for std::net::IpAddr {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for std::net::SocketAddr {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

// ## Implementors

impl<'r> OpenApiFromRequest<'r> for &'r rocket::config::Config {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

#[cfg(feature = "secrets")]
impl<'r> OpenApiFromRequest<'r> for &'r rocket::config::SecretKey {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r rocket::data::Limits {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r rocket::http::Accept {
    fn from_request_input(gen: &mut OpenApiGenerator, _name: String, required: bool) -> Result {
        let schema = gen.json_schema::<String>();
        Ok(RequestHeaderInput::Parameter(Parameter {
            name: "Accept".to_owned(),
            location: "header".to_owned(),
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
        }))
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r rocket::http::ContentType {
    fn from_request_input(gen: &mut OpenApiGenerator, _name: String, required: bool) -> Result {
        let schema = gen.json_schema::<String>();
        Ok(RequestHeaderInput::Parameter(Parameter {
            name: "Content-Type".to_owned(),
            location: "header".to_owned(),
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
        }))
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r rocket::http::CookieJar<'r> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r rocket::http::uri::Host<'r> {
    fn from_request_input(gen: &mut OpenApiGenerator, _name: String, required: bool) -> Result {
        let schema = gen.json_schema::<String>();
        Ok(RequestHeaderInput::Parameter(Parameter {
            name: "Host".to_owned(),
            location: "header".to_owned(),
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
        }))
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r rocket::http::uri::Origin<'r> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for &'r rocket::route::Route {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for rocket::http::Method {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

#[cfg(feature = "mtls")]
impl<'r> OpenApiFromRequest<'r> for rocket::mtls::Certificate<'r> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for rocket::Shutdown {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r> OpenApiFromRequest<'r> for rocket::request::FlashMessage<'r> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r, T: Send + Sync + 'static> OpenApiFromRequest<'r> for &'r rocket::State<T> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

impl<'r, T: OpenApiFromRequest<'r>> OpenApiFromRequest<'r> for Option<T> {
    fn from_request_input(gen: &mut OpenApiGenerator, name: String, _required: bool) -> Result {
        T::from_request_input(gen, name, false)
    }
}

impl<'r, T: OpenApiFromRequest<'r>> OpenApiFromRequest<'r> for StdResult<T, T::Error> {
    fn from_request_input(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        T::from_request_input(gen, name, required)
    }
}

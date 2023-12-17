use super::{OpenApiFromRequest, RequestHeaderInput};
use crate::gen::OpenApiGenerator;
use okapi::openapi3::*;
use std::result::Result as StdResult;

// Implement `OpenApiFromRequest` for everything that implements `FromRequest`
// Order is same as on:
// https://docs.rs/rocket/0.5.0/rocket/request/trait.FromRequest.html#foreign-impls
// https://api.rocket.rs/v0.5/rocket/request/trait.FromRequest.html#foreign-impls

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

impl<'r, T: OpenApiFromRequest<'r>> OpenApiFromRequest<'r>
    for rocket::request::Outcome<T, T::Error>
{
    fn from_request_input(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        T::from_request_input(gen, name, required)
    }
}

// ## Implementations for other crates
// https://docs.rs/rocket_db_pools/0.1.0/rocket_db_pools/struct.Connection.html#impl-FromRequest%3C%27r%3E

#[cfg(feature = "rocket_db_pools")]
impl<'r, D: rocket_db_pools::Database> OpenApiFromRequest<'r> for rocket_db_pools::Connection<D> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

#[cfg(feature = "rocket_dyn_templates")]
impl<'r> OpenApiFromRequest<'r> for rocket_dyn_templates::Metadata<'r> {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

#[cfg(feature = "rocket_sync_db_pools")]
impl<'r> OpenApiFromRequest<'r> for rocket_sync_db_pools::example::ExampleDb {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::None)
    }
}

#[cfg(feature = "rocket_ws")]
impl<'r> OpenApiFromRequest<'r> for rocket_ws::WebSocket {
    fn from_request_input(_gen: &mut OpenApiGenerator, _name: String, _required: bool) -> Result {
        Ok(RequestHeaderInput::Server(
            "ws://{server}/{base_path}".to_owned(),
            Some("WebSocket connection".to_owned()),
            okapi::map! {
                "server".to_owned() => okapi::openapi3::ServerVariable {
                    default: "127.0.0.1:8000".to_owned(),
                    ..Default::default()
                },
                "base_path".to_owned() => okapi::openapi3::ServerVariable {
                    default: "".to_owned(),
                    ..Default::default()
                },
            },
        ))
    }
}

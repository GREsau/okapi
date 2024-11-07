//! ------ Just Cookies (for just 1 route/endpoint) ------

use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::serde::json::Json;
use rocket::{
    get,
    request::{self, FromRequest},
};
use rocket_okapi::okapi::openapi3::{Object, Parameter, ParameterValue};
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

#[allow(dead_code)]
pub struct CookieAuth(String);

// Implement the actual checks for the authentication
#[rocket::async_trait]
impl<'a> FromRequest<'a> for CookieAuth {
    type Error = &'static str;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private("user_id") // Requires "secrets" feature flag
            .and_then(|cookie| cookie.value().parse().ok())
            .map(CookieAuth)
            .or_forward(Status::Unauthorized)
    }
}

impl<'a> OpenApiFromRequest<'a> for CookieAuth {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        _name: String,
        required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let schema = gen.json_schema::<String>();
        Ok(RequestHeaderInput::Parameter(Parameter {
            name: "user_id".to_owned(),
            location: "cookie".to_owned(),
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

/// # Just Cookies (for just 1 route/endpoint)
///
/// (make sure this is only sent over HTTPS, don't want secrets to leak)
///
/// Note: Cookies will not work with the `Try` button because of
/// [Technical limitations](https://developer.mozilla.org/en-US/docs/Glossary/Forbidden_header_name).
#[openapi]
#[get("/cookie_auth")]
pub fn cookie_auth(user: CookieAuth) -> Json<&'static str> {
    let _seems_you_have_access = user;
    Json("You got access")
}

//! ------ HTTP `Authorization` header ------

use rocket::serde::json::Json;
use rocket::{
    get,
    http::Status,
    request::{self, FromRequest, Outcome},
};
use rocket_okapi::okapi::openapi3::{
    Object, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

#[allow(dead_code)]
pub struct HttpAuth(String);

// Implement the actual checks for the authentication
#[rocket::async_trait]
impl<'a> FromRequest<'a> for HttpAuth {
    type Error = &'static str;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        // Get the token from the http header
        match request.headers().get_one("Authorization") {
            Some(token) => {
                if token == "Bearer mytoken" {
                    Outcome::Success(HttpAuth(token.to_owned()))
                } else {
                    Outcome::Error((Status::Unauthorized, "Auth is invalid."))
                }
            }
            None => Outcome::Error((Status::BadRequest, "Missing `Authorization` header.")),
        }
        // For more info see: https://rocket.rs/v0.5/guide/state/#within-guards
    }
}

impl<'a> OpenApiFromRequest<'a> for HttpAuth {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an Bearer token to access, token is: `mytoken`.".to_owned(),
            ),
            // Setup data requirements.
            // In this case the header `Authorization: mytoken` needs to be set.
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(), // `basic`, `digest`, ...
                // Just gives use a hint to the format used
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}

/// # HTTP `Authorization` header
///
/// The token is: `mytoken`
/// This is a common way of checking the authentication.
/// (make sure this is only sent over HTTPS, don't want secrets to leak)
#[openapi]
#[get("/http_auth")]
pub fn http_auth(token: HttpAuth) -> Json<&'static str> {
    // Use api key
    let _seems_you_have_access = token;
    Json("You got access")
}

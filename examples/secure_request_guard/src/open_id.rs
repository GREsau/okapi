//! ------ OpenID Connect ------

use rocket::serde::json::Json;
use rocket::{
    get,
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
pub struct OpenId(String);

// Implement the actual checks for the authentication
#[rocket::async_trait]
impl<'a> FromRequest<'a> for OpenId {
    type Error = &'static str;
    async fn from_request(
        _request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        // Implement read OpenId flow here, this is to much work for an example.
        // Good luck!
        Outcome::Success(OpenId("Some info".to_owned()))
    }
}

impl<'a> OpenApiFromRequest<'a> for OpenId {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Use OpenID Connect to authenticate. (does not work in RapiDoc at all)".to_owned(),
            ),
            data: SecuritySchemeData::OpenIdConnect {
                open_id_connect_url:
                    "https://demo.identityserver.io/.well-known/openid-configuration".to_owned(),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("OpenID".to_owned(), Vec::new());
        // These vvvv-------^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "OpenID".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}

/// # OpenID Connect
///
/// This is not implemented, so this will not work correctly.
#[openapi]
#[get("/open_id")]
pub fn open_id(data: OpenId) -> Json<&'static str> {
    // Use data
    let _seems_you_have_access = data;
    Json("You got access")
}

//! ------ OAuth 2.0 flows (authorizationCode, implicit, password, clientCredentials) ------
use rocket::serde::json::{json, Json};
use rocket::{
    get,
    http::Status,
    request::{self, FromRequest, Outcome},
};
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{
    OAuthFlows, Object, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

pub struct OAuth2AuthCode;

// Implement the actual checks for the authentication
#[rocket::async_trait]
impl<'a> FromRequest<'a> for OAuth2AuthCode {
    type Error = &'static str;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        // Get the jwt from the http header
        match request.headers().get_one("Authorization") {
            Some(jwt) => {
                // TODO Use proper OAuth2/JWT verification here. (This is just en example)
                if jwt.starts_with("Bearer ") {
                    Outcome::Success(OAuth2AuthCode)
                } else {
                    Outcome::Error((Status::Unauthorized, "JWT is invalid."))
                }
            }
            None => Outcome::Error((Status::BadRequest, "Missing `Authorization` header.")),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for OAuth2AuthCode {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an API key to access, \
                client_id = `interactive.confidential`, \
                client_secret = `secret`. \
                (Does not work in Swagger UI)"
                    .to_owned(),
            ),
            // Setup data requirements.
            data: SecuritySchemeData::OAuth2 {
                // Other flows are very similar.
                // For more info see: https://swagger.io/docs/specification/authentication/oauth2/
                flows: OAuthFlows::AuthorizationCode {
                    authorization_url: "https://demo.identityserver.io/connect/authorize"
                        .to_owned(),
                    token_url: "https://demo.identityserver.io/connect/token".to_owned(),
                    refresh_url: None,
                    scopes: okapi::map! {
                        "openid".to_owned() => "Ability to access openid".to_owned(),
                        "profile".to_owned() => "Ability to access profile".to_owned(),
                        "email".to_owned() => "Ability to access email".to_owned(),
                        "api".to_owned() => "Ability to use api".to_owned(),
                        "offline_access".to_owned() => "Ability for offline access".to_owned(),
                    },
                    extensions: Object::default(),
                },
            },
            // Add example data for RapiDoc
            extensions: okapi::map! {
                "x-client-id".to_owned() => json!("interactive.confidential"),
                "x-client-secret".to_owned() => json!("secret"),
            },
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("OAuth2AuthCode".to_owned(), vec!["profile".to_owned()]);
        // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "OAuth2AuthCode".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}

/// # OAuth 2.0 flow: Authorization Code
///
/// This endpoint requires `profile` scope. (not checked)
#[openapi]
#[get("/oauth2_auth_code/get_user")]
pub fn oauth2_auth_code_get_user(auth: OAuth2AuthCode) -> Json<&'static str> {
    let _seems_you_have_access = auth;
    Json("You got access")
}

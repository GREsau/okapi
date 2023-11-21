//! ------ No special authentication ------

use rocket::serde::json::Json;
use rocket::{
    get,
    request::{self, FromRequest, Outcome},
};
use rocket_okapi::{openapi, request::OpenApiFromRequest};

#[derive(OpenApiFromRequest)]
pub struct NoSpecialAuthentication;

#[rocket::async_trait]
impl<'a> FromRequest<'a> for NoSpecialAuthentication {
    type Error = &'static str;
    async fn from_request(
        _request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(NoSpecialAuthentication)
    }
}

/// # No special authentication
/// This is most often used then you have something that it not related to authentication.
/// For example database connections, or other managed (static) data.
/// <https://rocket.rs/v0.5/guide/state/>
#[openapi]
#[get("/no_auth")]
pub fn no_special_auth(something: NoSpecialAuthentication) -> Json<&'static str> {
    // Use `something` here
    let _use_value = something;
    Json("No special authentication needed.")
}

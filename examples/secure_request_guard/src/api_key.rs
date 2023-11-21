//! ------ ApiKey (in http header, query or cookie) ------
use rocket::serde::json::Json;
use rocket::{
    get,
    http::Status,
    request::{self, FromRequest, Outcome},
};
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{
    Object, Responses, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

pub struct ApiKey(String);

// Implement the actual checks for the authentication
#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey {
    type Error = &'static str;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        // Get the key from the http header
        match request.headers().get_one("x-api-key") {
            Some(key) => {
                if key == "mykey" {
                    Outcome::Success(ApiKey(key.to_owned()))
                } else {
                    Outcome::Error((Status::Unauthorized, "Api key is invalid."))
                }
            }
            None => Outcome::Error((Status::BadRequest, "Missing `x-api-key` header.")),
        }
        // For more info see: https://rocket.rs/v0.5/guide/state/#within-guards
    }
}

impl<'a> OpenApiFromRequest<'a> for ApiKey {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some("Requires an API key to access, key is: `mykey`.".to_owned()),
            // Setup data requirements.
            // This can be part of the `header`, `query` or `cookie`.
            // In this case the header `x-api-key: mykey` needs to be set.
            data: SecuritySchemeData::ApiKey {
                name: "x-api-key".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("ApiKeyAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }

    // Optionally add responses
    // Also see `main.rs` part of this.
    fn get_responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        use rocket_okapi::okapi::openapi3::RefOr;
        // Can switch between to the but both are checked if they compile correctly
        let use_method = "recommended";
        // It can return a "400 BadRequest" and a "401 Unauthorized"
        // In both cases we just return a what we have set in the catches (if any).
        // In our cases this is: `crate::MyError`
        // This depends on you catcher return type.

        // Below are 3 examples, all are similar, first 2 are recommended.
        match use_method {
            "recommended" => Ok(Responses {
                // Recommended and most strait forward.
                // And easy to add or remove new responses.
                responses: okapi::map! {
                    "400".to_owned() => RefOr::Object(crate::bad_request_response(gen)),
                    "401".to_owned() => RefOr::Object(crate::unauthorized_response(gen)),
                },
                ..Default::default()
            }),
            "1st alternative" => {
                // This is same as macro above does, so just depends on what you like more.
                let mut responses = Responses::default();
                responses.responses.insert(
                    "400".to_owned(),
                    RefOr::Object(crate::bad_request_response(gen)),
                );
                responses.responses.insert(
                    "401".to_owned(),
                    RefOr::Object(crate::unauthorized_response(gen)),
                );
                Ok(responses)
            }
            "2nd alternative" => {
                // This not advised because of issue #57.
                // But this does work.
                // https://github.com/GREsau/okapi/issues/57
                // Note: this one does not add the `description` field to the responses.
                // So it is slightly different in output.
                let mut responses = Responses::default();
                let schema = gen.json_schema::<crate::MyError>();
                // Add "400 BadRequest"
                rocket_okapi::util::add_schema_response(
                    &mut responses,
                    400,
                    "application/json",
                    schema.clone(),
                )?;
                // Add "401 Unauthorized"
                rocket_okapi::util::add_schema_response(
                    &mut responses,
                    401,
                    "application/json",
                    schema,
                )?;
                Ok(responses)
            }
            _ => Ok(Responses::default()),
        }
    }
}

/// # ApiKey (in http header, query or cookie)
///
/// The key is: `mykey`
/// This is a common way of checking the authentication.
/// (make sure this is only sent over HTTPS, don't want secrets to leak)
///
/// Using `query` is not recommended for secrets!
/// For more info see:
/// <https://owasp.org/www-community/vulnerabilities/Information_exposure_through_query_strings_in_url>
#[openapi]
#[get("/apikey")]
pub fn api_key(key: ApiKey) -> Result<Json<&'static str>, crate::MyError> {
    // Use api key
    let _seems_you_have_access = key;
    Ok(Json("You got access"))
}

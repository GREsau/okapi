use okapi::openapi3::{
    Object, Responses, SchemeIdentifier, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket::serde::json::Json;
use rocket::{
    get,
    http::Status,
    request::{self, FromRequest, Outcome},
    Config, Response,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi, openapi_get_routes,
    request::{OpenApiFromRequest, RequestHeaderInput},
    response::OpenApiResponder,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

pub struct KeyAuthorize;

#[rocket::async_trait]
impl<'a> FromRequest<'a> for KeyAuthorize {
    type Error = ();
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        // Same as in the name
        let keys: Vec<_> = request.headers().get("x-api-key").collect();

        // Get the key from the http header
        let out = match keys.len() {
            1 => {
                let key = &keys[0][..];
                if key == "hello" {
                    Outcome::Success(KeyAuthorize)
                } else {
                    Outcome::Failure((Status::Unauthorized, ()))
                }
            }
            _ => {
                println!("wrong amount of authorization headers found");
                Outcome::Failure((Status::BadRequest, ()))
            }
        };
        out
    }
}

impl<'a, 'r> OpenApiFromRequest<'a> for KeyAuthorize {
    fn request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut security_req = SecurityRequirement::new();
        // each security requirement needs a specific key in the openapi docs
        security_req.insert("example_security".into(), Vec::new());

        // The scheme for the security needs to be defined as well
        // https://swagger.io/docs/specification/authentication/basic-authentication/
        let security_scheme = SecurityScheme {
            description: Some("requires a key to access".into()),
            // this will show where and under which name the value will be found in the HTTP header
            // in this case, the header key x-api-key will be searched
            // other alternatives are "query", "cookie" according to the openapi specs.
            // [link](https://swagger.io/specification/#security-scheme-object)
            // which also is where you can find examples of how to create a JWT scheme for example
            data: SecuritySchemeData::ApiKey {
                name: "x-api-key".into(),
                location: "header".into(),
            },
            extensions: Object::default(),
        };

        // scheme identifier is the keyvalue under which this security_scheme will be filed in
        // the openapi.json file
        let scheme_identifier = SchemeIdentifier {
            scheme_identifier: "FixedKeyApiKeyAuth".into(),
        };

        Ok(RequestHeaderInput::Security((
            security_scheme,
            security_req,
            scheme_identifier,
        )))
    }
}

/// Defines the possible responses for this request guard for the openapi docs (not used yet)
impl<'a, 'r: 'a> OpenApiResponder<'a, 'r> for KeyAuthorize {
    fn responses(_: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let responses = Responses::default();
        Ok(responses)
    }
}

/// Returns an empty, default `Response`. Always returns `Ok`.
/// Defines the possible response for this request guard
impl<'a, 'r: 'a> rocket::response::Responder<'a, 'r> for KeyAuthorize {
    fn respond_to(self, _: &rocket::request::Request<'_>) -> rocket::response::Result<'static> {
        Ok(Response::new())
    }
}

#[openapi]
#[get("/restricted")]
pub fn restricted(_key: KeyAuthorize) -> Json<String> {
    Json("You got access here, hurray".into())
}

#[tokio::main]
async fn main() {
    let rocket_config = Config::debug_default();

    let e = rocket::custom(rocket_config)
        .mount("/", openapi_get_routes![restricted])
        .mount(
            "/api/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/openapi.json".to_string(),
                ..Default::default()
            }),
        )
        .launch()
        .await;
}

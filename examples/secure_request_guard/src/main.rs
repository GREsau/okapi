use rocket::Request;
use rocket::{catch, catchers, response, response::Responder, Response};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi_get_routes, rapidoc::*, swagger_ui::*};

// --------- All different methods of implementing `OpenApiFromRequest` ------------
// There are a few different ways of doing things.
// And it also depend on the authentication (if any) you want to implement.
// Here are a few different example that cover most of the use cases:
// - No special authentication
// - ApiKey (in http header, query or cookie)
// - HTTP `Authorization` header (inc `basic`, `digest` and `bearer` tokens)
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication#authentication_schemes
// - OAuth 2.0 flows (authorizationCode, implicit, password, clientCredentials)
// - OpenID Connect
// - Just Cookies (for just 1 route/endpoint)
// ---------------------------------------------------------------------------------

mod no_auth;

mod api_key;

mod http_auth;

mod oauth2;

mod open_id;

mod cookies;

#[tokio::main]
async fn main() {
    let launch_result = rocket::build()
        .mount(
            "/",
            openapi_get_routes![
                no_auth::no_special_auth,
                api_key::api_key,
                http_auth::http_auth,
                oauth2::oauth2_auth_code_get_user,
                open_id::open_id,
                cookies::cookie_auth,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Dark,
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .register("/", catchers![bad_request, unauthorized])
        .launch()
        .await;
    match launch_result {
        Ok(()) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

// ----- Catchers -------

/// Error messages returned to user
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
struct MyError {
    /// The title of the error message
    pub err: String,
    /// The description of the error
    pub msg: Option<String>,
    // HTTP Status Code returned
    #[serde(skip)]
    pub http_status_code: u16,
}

#[catch(400)]
fn bad_request() -> MyError {
    MyError {
        err: "Bad Request".to_owned(),
        msg: Some("The request given is wrongly formatted or data was missing.".to_owned()),
        http_status_code: 400,
    }
}

#[catch(401)]
fn unauthorized() -> MyError {
    MyError {
        err: "Unauthorized".to_owned(),
        msg: Some("The authentication given was incorrect or insufficient.".to_owned()),
        http_status_code: 401,
    }
}

impl<'r> Responder<'r, 'static> for MyError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .status(rocket::http::Status::new(self.http_status_code))
            .ok()
    }
}

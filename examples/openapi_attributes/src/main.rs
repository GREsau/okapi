use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::{get, serde::json::Json};
use rocket_okapi::okapi::schemars;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct User {
    name: String,
}

#[openapi(tag = "Users", ignore = "db")]
#[get("/users")]
fn list_users(db: CustomDB) -> Json<Vec<User>> {
    Json(db.users)
}

// VERY simple fake read only database
#[derive(Clone, Default)]
struct CustomDB {
    users: Vec<User>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CustomDB {
    type Error = String;

    async fn from_request(_request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(CustomDB {
            users: vec![User {
                name: "Bob".to_owned(),
            }],
        })
    }
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![list_users])
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
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

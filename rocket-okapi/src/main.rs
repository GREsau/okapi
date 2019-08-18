#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate schemars;

use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::*;
use rocket_okapi::OpenApiError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    username: String,
    #[serde(default)]
    email: Option<String>,
}

#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[openapi]
#[get("/user")]
fn get_user() -> Option<Json<User>> {
    Some(Json(User {
        username: "bob".to_owned(),
        user_id: 12345,
        email: None,
    }))
}

#[openapi]
#[post("/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[openapi]
#[get("/500")]
fn five_hundred() -> Result<&'static str, OpenApiError> {
    Err(OpenApiError::new("OH NO!".to_owned()))
}

#[get("/hidden")]
#[openapi(skip)]
fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes_with_openapi![index, get_user, create_user, hidden, five_hundred],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: Some("/openapi/openapi.json".to_owned()),
                urls: None,
            }),
        )
        .launch();
}

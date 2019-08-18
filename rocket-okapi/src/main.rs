#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate schemars;

use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::*;
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
#[get("/user")]
fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

#[openapi]
#[get("/user/<id>")]
fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

#[openapi]
#[post("/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
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
            routes_with_openapi![get_all_users, get_user, create_user, hidden],
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

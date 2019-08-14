#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate schemars;

use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use rocket_okapi::gen::{OpenApiGenerator, OpenApiSettings};
use rocket_okapi::handler::ContentHandler;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    username: String,
    #[serde(default)]
    email: Option<String>,
}

#[openapi]
#[get("/")]
fn index() -> Json<&'static str> {
    Json("Hello, world!")
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
#[get("/loud")]
fn loud() -> Json<Option<&'static str>> {
    Json(Some("I AM SHOUTING!!!!!"))
}

#[openapi(skip)]
#[get("/tonumber/<value>")]
fn to_number(value: String) -> Result<Json<f64>, NotFound<Json<&'static str>>> {
    match value.parse() {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(NotFound(Json("That's not a number!"))),
    }
}

#[openapi(skip)]
#[post("/tonumber", data = "<value>")]
fn to_number_post(value: Json<String>) -> Result<Json<f64>, NotFound<Json<&'static str>>> {
    match value.parse() {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(NotFound(Json("That's not a number!"))),
    }
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
            routes_with_openapi![index, loud, to_number, to_number_post, hidden, get_user],
        )
        .launch();
}

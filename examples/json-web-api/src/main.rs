#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

use rocket::request::{Form, FromForm};
use rocket_contrib::json::Json;
use rocket_okapi::{settings::OpenApiSettings, swagger_ui::*};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use okapi::openapi3::SecuritySchemeData;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    username: String,
    #[serde(default)]
    #[schemars(example = "example_email")]
    email: Option<String>,
}

fn example_email() -> &'static str {
    "test@example.com"
}

/// # Get all users
///
/// Returns all users in the system.
#[openapi(security = "x-api-key")]
#[get("/user")]
fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: 42,
        username: "bob".to_owned(),
        email: None,
    }])
}

/// # Get user
///
/// Returns a single user by ID.
#[openapi]
#[get("/user/<id>")]
fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

/// # Get user by name
///
/// Returns a single user by username.
#[openapi]
#[get("/user_example?<user_id>&<name>&<email>")]
fn get_user_by_name(user_id: u64, name: String, email: Option<String>) -> Option<Json<User>> {
    Some(Json(User {
        user_id,
        username: name,
        email,
    }))
}

/// # Create user
#[openapi]
#[post("/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[openapi(skip)]
#[get("/hidden")]
fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

#[derive(Serialize, Deserialize, JsonSchema, FromForm)]
struct Post {
    /// The unique identifier for the post.
    post_id: u64,
    /// The title of the post.
    title: String,
    /// A short summary of the post.
    summary: Option<String>,
}

/// # Create post using query params
///
/// Returns the created post.
#[openapi]
#[get("/post_by_query?<post..>")]
fn create_post_by_query(post: Form<Post>) -> Option<Json<Post>> {
    Some(Json(post.into_inner()))
}

fn main() {
    let mut settings = OpenApiSettings::default();
    settings.add_security_scheme("x-api-key".to_owned(), SecuritySchemeData::ApiKey {name: "x-api-key".to_owned(),location: "header".to_owned()},None);
    rocket::ignite()
        .mount(
            "/",
            routes_with_openapi_with_settings![
                get_all_users,
                get_user,
                get_user_by_name,
                create_user,
                hidden,
                create_post_by_query,
                settings
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch();
}

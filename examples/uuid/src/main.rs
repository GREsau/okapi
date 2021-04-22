#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

use rocket::serde::{json::Json, uuid::Uuid};
use rocket_okapi::swagger_ui::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: uuid::Uuid,
}

/// # Get all users
///
/// Returns all users in the system.
#[openapi]
#[get("/user")]
fn get_all_users() -> Json<Vec<User>> {
    Json(vec![User {
        user_id: uuid::Uuid::new_v4(),
    }])
}

/// # Get user
///
/// Returns a single user by ID.
#[openapi]
#[get("/user/<id>")]
fn get_user(id: Uuid) -> Option<Json<User>> {
    Some(Json(User { user_id: id }))
}

/// # Get user by name
///
/// Returns a single user by username.
#[openapi]
#[get("/user_example?<id>")]
fn get_user_by_name(id: Uuid) -> Option<Json<User>> {
    Some(Json(User { user_id: id }))
}

/// # Create user
#[openapi]
#[post("/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}

#[rocket::main]
async fn main() {
    let result = rocket::build()
        .mount(
            "/",
            routes_with_openapi![get_all_users, get_user, get_user_by_name, create_user,],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await;
    assert!(result.is_ok());
}

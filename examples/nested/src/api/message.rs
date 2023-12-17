use rocket::form::FromForm;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use rocket_okapi::openapi;
use rocket_okapi::openapi_get_routes_spec;
use rocket_okapi::settings::OpenApiSettings;
use serde::{Deserialize, Serialize};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_message, get_message]
}

#[derive(Serialize, Deserialize, JsonSchema, FromForm)]
struct Message {
    /// The unique identifier for the message.
    message_id: u64,
    /// Content of the message.
    content: String,
}

/// # Create a message
///
/// Returns the created message.
#[openapi(tag = "Message")]
#[post("/", data = "<message>")]
fn create_message(message: crate::DataResult<'_, Message>) -> crate::Result<Message> {
    let message = message?.into_inner();
    Ok(Json(message))
}

/// # Get a message by id
///
/// Returns the message with the requested id.
#[openapi(tag = "Message")]
#[get("/<id>")]
fn get_message(id: u64) -> crate::Result<Message> {
    Ok(Json(Message {
        message_id: id,
        content: "Hey, how are you?".to_owned(),
    }))
}

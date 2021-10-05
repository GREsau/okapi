use rocket::form::FromForm;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use rocket_okapi::openapi;
use rocket_okapi::openapi_get_routes_spec;
use rocket_okapi::settings::OpenApiSettings;
use serde::{Deserialize, Serialize};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_post, get_post]
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

/// # Create post
///
/// Returns the created post.
#[openapi(tag = "Posts")]
#[post("/", data = "<post>")]
fn create_post(post: crate::DataResult<'_, Post>) -> crate::Result<Post> {
    let post = post?.into_inner();
    Ok(Json(post))
}

/// # Get a post by id
///
/// Returns the post with the requested id.
#[openapi(tag = "Posts")]
#[get("/<id>")]
fn get_post(id: u64) -> crate::Result<Post> {
    Ok(Json(Post {
        post_id: id,
        title: "Your post".to_owned(),
        summary: Some("Best summary ever.".to_owned()),
    }))
}

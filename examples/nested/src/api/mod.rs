mod post;
mod message;

use rocket::Route;
use rocket_okapi::{okapi::openapi3::OpenApi, settings::OpenApiSettings, get_nested_endpoints_and_docs};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/posts" => post::get_routes_and_docs(settings),
        "/message" => message::get_routes_and_docs(settings),
    }
}

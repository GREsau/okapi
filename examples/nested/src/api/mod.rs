mod message;
mod post;

use rocket_okapi::{
    get_nested_endpoints_and_docs, okapi::openapi3::OpenApi, settings::OpenApiSettings,
};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/posts" => post::get_routes_and_docs(settings),
        "/message" => message::get_routes_and_docs(settings),
    }
}

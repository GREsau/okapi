mod post;
mod message;

use rocket::Route;
use rocket_okapi::{okapi::openapi3::OpenApi, settings::OpenApiSettings};

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    let mut routes = vec![];
    let mut openapi_list: Vec<(_, rocket_okapi::okapi::openapi3::OpenApi)> = Vec::new();

    [
        ("/posts", post::get_routes_and_docs(settings)),
        ("/message", message::get_routes_and_docs(settings)),
    ]
    .into_iter()
    .for_each(|(path, (new_routes, openapi))| {
        let new_routes = new_routes
            .into_iter()
            .map(|r: Route| r.map_base(|base| format!("{}{}", path, base)).unwrap())
            .collect::<Vec<_>>();
        routes.extend(new_routes);
        openapi_list.push((path, openapi));
    });

    let openapi_docs = match rocket_okapi::okapi::merge::marge_spec_list(&openapi_list) {
        Ok(docs) => docs,
        Err(err) => panic!("Could not merge OpenAPI spec: {}", err),
    };

    (routes, openapi_docs)
}

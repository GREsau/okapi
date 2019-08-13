#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use rocket_okapi::gen::{OpenApiGenerator, OpenApiSettings};
use rocket_okapi::handler::ContentHandler;

#[openapi]
#[get("/")]
fn index() -> Json<&'static str> {
    Json("Hello, world!")
}

#[openapi]
#[get("/loud")]
fn loud() -> Json<Option<&'static str>> {
    Json(Some("I AM SHOUTING!!!!!"))
}

// #[openapi]
#[get("/tonumber/<value>")]
fn to_number(value: String) -> Result<Json<f64>, NotFound<Json<&'static str>>> {
    match value.parse() {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(NotFound(Json("That's not a number!"))),
    }
}

// #[openapi]
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
        .mount("/", {
            let settings = OpenApiSettings::new();
            let mut gen = OpenApiGenerator::new(settings.clone());
            _okapi_add_operation_for_index_(&mut gen, "index".to_owned())
                .expect("Could not generate OpenAPI operation for `index`.");
            _okapi_add_operation_for_loud_(&mut gen, "loud".to_owned())
                .expect("Could not generate OpenAPI operation for `loud`.");
            _okapi_add_operation_for_hidden_(&mut gen, "hidden".to_owned())
                .expect("Could not generate OpenAPI operation for `hidden`.");
            let spec = gen.into_openapi();

            let mut routes = routes![index, loud, to_number, to_number_post, hidden];
            routes.push(ContentHandler::json(&spec).into_route(&settings.json_path));
            routes
        })
        .launch();
    // rocket::ignite().mount("/", routes_with_openapi![settings => index, loud, to_number, to_number_post, hidden]).launch();
}

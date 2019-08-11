#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use rocket_okapi::gen::{OpenApiGenerator, OpenApiSettings};
use rocket_okapi::handler::ContentHandler;
use rocket_okapi::OpenApiResponses;

//#[macro_use]
//extern crate rocket_okapi;

#[get("/")]
//#[okapi]
fn index() -> Json<&'static str> {
    Json("Hello, world!")
}

fn okapi_add_operation_index(
    gen: &mut ::rocket_okapi::gen::OpenApiGenerator,
) -> ::rocket_okapi::Result<()> {
    let responses = <Json<&'static str>>::responses(gen)?;
    gen.add_operation(::rocket_okapi::OperationInfo {
        path: "/".to_owned(),
        method: ::rocket::http::Method::Get,
        operation: ::okapi::openapi3::Operation {
            operation_id: Some("index".to_owned()),
            responses,
            ..Default::default()
        },
    });
    Ok(())
}

#[get("/loud")]
//#[okapi]
//#[okapi(200 => &str)]
//#[okapi(404 => ())]
//#[okapi(401 => (), "Authentication failed.")]
fn loud() -> Json<&'static str> {
    Json("I AM SHOUTING!!!!!")
}

#[get("/tonumber/<value>")]
//#[okapi]
fn to_number(value: String) -> Result<Json<f64>, NotFound<Json<&'static str>>> {
    match value.parse() {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(NotFound(Json("That's not a number!"))),
    }
}

#[post("/tonumber", data = "<value>")]
//#[okapi]
fn to_number_post(value: Json<String>) -> Result<Json<f64>, NotFound<Json<&'static str>>> {
    match value.parse() {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(NotFound(Json("That's not a number!"))),
    }
}

#[get("/hidden")]
//#[okapi(skip)]
fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

fn routes_with_openapi() -> Vec<rocket::Route> {
    let mut gen = OpenApiGenerator::new(OpenApiSettings::new());
    okapi_add_operation_index(&mut gen).expect("Could not generate OpenAPI operation for `index`.");
    let spec = gen.into_openapi();

    let mut routes = routes![index, loud, to_number, to_number_post, hidden];
    routes.push(ContentHandler::json(&spec).into_route("/swagger.json"));
    routes
}

fn main() {
    rocket::ignite().mount("/", routes_with_openapi()).launch();
}

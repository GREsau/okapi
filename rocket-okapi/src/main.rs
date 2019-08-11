#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::status::NotFound;
use rocket_contrib::json::Json;

//#[macro_use]
//extern crate rocket_okapi;

#[get("/")]
//#[okapi]
fn index() -> Json<&'static str> {
    Json("Hello, world!")
}

/*fn okapi_make_operation_info_index() -> Option<::rocket_okapi::OperationInfo> {
    Some(::rocket_okapi::OperationInfo {
        path: "/".to_owned(),
        method: ::rocket::http::Method::Get,
        operation: ::okapi::openapi3::Operation {
            operation_id: Some("index".to_owned()),
            responses: ::okapi::openapi3::Responses {
                responses: {
                    let mut map = ::okapi::Map::new();
                    map.insert(
                        "200".to_owned(),
                        ::okapi::openapi3::Response {
                            content: {
                                let mut map = ::okapi::Map::new();
                                map.insert(
                                    "application/json".to_owned(),
                                    ::okapi::openapi3::MediaType {
                                        schema: ,
                                        ..Default::default()
                                    },
                                );
                                map
                            },
                            ..Default::default()
                        }
                        .into(),
                    );
                    map
                },
                ..Default::default()
            },
            ..Default::default()
        },
    })
}*/

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

fn main() {
    // let okapi = OkapiGenerator::new().mount("/", okapi_routes![index, loud, to_number, to_number_post, hidden]).generate("Test API", "0.1");
    rocket::ignite()
        // .mount_okapi("/swagger", okapi)
        // or .mount_okapi("/swagger", okapi_routes![index, loud, to_number, to_number_post, hidden])
        .mount("/", routes![index, loud, to_number, to_number_post, hidden])
        .launch();
}

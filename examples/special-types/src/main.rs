use rocket::http::Status;
use rocket::{get, post, serde::json::Json};
use rocket_okapi::okapi::schemars;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// # Get data
#[openapi(tag = "Users")]
#[post("/get_date", data = "<req_body>")]
fn get_data(req_body: Json<String>) -> Option<Json<()>> {
    let _ = req_body;
    Some(Json(()))
}

#[openapi]
#[get("/paths/<path..>")]
fn path_info(path: PathBuf) -> (rocket::http::Status, String) {
    (rocket::http::Status::ImATeapot, format!("info {:?}", path))
}

#[openapi(tag = "Users")]
#[post("/user", data = "<req_body>", format = "application/json")]
fn create_user(req_body: Json<String>) -> Result<Json<User>, (Status, Json<ErrorMessage>)> {
    let _ = req_body;
    Ok(Json(User {
        name: "bob".to_owned(),
    }))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct User {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ErrorMessage {
    pub message: String,
    pub code: u16,
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![get_data, path_info, create_user])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(()) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

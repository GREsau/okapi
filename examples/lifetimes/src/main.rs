use rocket::{Build, get, Rocket, State};
use rocket_okapi::{mount_endpoints_and_merged_docs, openapi, openapi_get_routes_spec};
use rocket_okapi::rapidoc::{GeneralConfig, HideShowConfig, make_rapidoc, RapiDocConfig};
use rocket_okapi::settings::{OpenApiSettings, UrlObject};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[rocket::main]
async fn main() {
    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

pub fn create_server() -> Rocket<Build> {
    let mut building_rocket = rocket::build()
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("My special documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );


    let (route, spec) = openapi_get_routes_spec![get_with_lifetimes];


    building_rocket.manage(Test).mount("/v1",route).mount("/v1/", vec![rocket_okapi::handlers::OpenApiHandler::new(spec).into_route(OpenApiSettings::new().json_path)])
}

struct Test;

#[openapi]
#[get("/get/<val>")]
fn get_with_lifetimes<'a>(state: &'a State<Test>, val: bool) -> String {
    val.to_string()
}

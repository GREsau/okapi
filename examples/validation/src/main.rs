
use rocket::serde::json::Json;
use rocket::{post, FromForm};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use schemars::JsonSchema;
use serde::Deserialize;
use rocket_validation::{Validate, Validated};

#[derive(Debug,Validate, Deserialize,JsonSchema,FromForm)]
#[serde(rename_all="camelCase")]
#[serde(crate = "rocket::serde")]
pub struct SampleForm {
    #[validate(length(min = 2, max = 25))]
    pub name: String,
    #[validate(range(min = 1, max = 200))]
    pub age: u8,
    #[validate(email)]
    pub email: String,
    #[validate(url)]
    pub website: String,
    pub phone: String,
}

#[openapi]
#[post("/sample-validation", format = "application/json",data = "<data>")]
fn create_sample_form(data: Validated<Json<SampleForm>>,) -> Json<String> {
   Json("Form created successfully".to_string() )   
}


#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![create_sample_form,])
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
                    allow_spec_file_download: true,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

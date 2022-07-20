#[macro_use] extern crate rocket;

use rocket::{Build, Config, Rocket};
use rocket::config::LogLevel;
use rocket::http::Status;
use rocket_okapi::{openapi_get_routes, openapi};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};



fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}


#[openapi]
#[get("/")]
pub(crate) async fn test_index() -> Result<String, Status> {
    Ok("I am here as a dummy route for the OpenAPI spec".to_string())
}


#[launch]
fn rocket() -> Rocket<Build> {
    let config = Config {
        log_level : LogLevel::Normal,
        ..Config::debug_default()
    };

    rocket::custom(&config)
        .mount("/", openapi_get_routes![test_index])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}

#[cfg(test)]
mod expected_unit_test {
    use rocket::serde::json::serde_json;
    use rocket_okapi::okapi::openapi3::OpenApi;
    use rocket_okapi::openapi_get_spec;
    use pretty_assertions::{assert_eq};
    use crate::*;

    #[test]
    fn openapi_cmp() {
        let ref_data = std::fs::read_to_string("./tests/expected/openapi.json").unwrap();
        let expected_spec : OpenApi = serde_json::from_str(&ref_data).unwrap();
        let current_spec : OpenApi = openapi_get_spec![test_index];
        assert_eq!(expected_spec, current_spec);
    }

    #[test]
    fn string_cmp() {
        // Expected to ALWAYS pass. Included to ensure the expected-file is correct.

        let ref_data = std::fs::read_to_string("./tests/expected/openapi.json").unwrap();
        let current_spec: OpenApi = openapi_get_spec![test_index];
        /* Note:
            Depending on whether or not the reference file is pretty printed we have to use to_string_pretty or to_string.

            The "better" way would be to deserialize ref_data to type OpenApi, however currently the deserialization is not working as expected
            due to an error with flattening some 'extensions' fields in structs of okapi::openapi3.
        */
        let current_str = serde_json::to_string_pretty(&current_spec).unwrap();
        // below: not-pretty-printed version
        // let current_str = serde_json::to_string(&current_spec).unwrap();
        assert_eq!(ref_data, current_str);
    }
}
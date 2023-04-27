//! This test ensures that routes can be marked as deprecated by using `#[openapi(deprecated)]`.

use rocket_okapi::openapi_get_spec;

// These functions are never actually called.
#[allow(unused)]
mod endpoints {
    use rocket::{get, serde::json::Json};
    use rocket_okapi::openapi;

    #[openapi(deprecated = true)]
    #[get("/explicit")]
    pub fn explicit_controller() -> Json<()> {
        Json(())
    }

    #[openapi(deprecated)]
    #[get("/implicit")]
    pub fn implicit_controller() -> Json<()> {
        Json(())
    }

    #[openapi]
    #[get("/default")]
    pub fn default_controller() -> Json<()> {
        Json(())
    }
}

#[test]
fn deprecated_with_explicit_value_is_deprecated() {
    let spec = openapi_get_spec![endpoints::explicit_controller];

    let operation = spec.paths["/explicit"].get.as_ref().unwrap();
    assert!(operation.deprecated);
}

#[test]
fn deprecated_with_implicit_value_is_deprecated() {
    let spec = openapi_get_spec![endpoints::implicit_controller];

    let operation = spec.paths["/implicit"].get.as_ref().unwrap();
    assert!(operation.deprecated);
}

#[test]
fn default_is_not_deprecated() {
    let spec = openapi_get_spec![endpoints::default_controller];

    let operation = spec.paths["/default"].get.as_ref().unwrap();
    assert!(!operation.deprecated);
}

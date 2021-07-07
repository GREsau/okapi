# okapi
OpenAPI (AKA Swagger) document generation for Rust projects forked version to enable some new features

only change your usage in Cargo toml to something like this by adding the "package" parameter:

```toml
okapi = { version = "0.x.x", features = ["derive_json_schema"], package = "okapi_fork" }
rocket_okapi = { version = "0.x.x", package = "rocket_okapi_fork" }

```

Work in progress!

## Basic Usage

```rust
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;

use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::make_swagger_ui;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Derive JsonSchema for and request/response models
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u64,
    username: String,
    #[serde(default)]
    email: Option<String>,
}

// Add #[openapi] attribute to your routes
#[openapi]
#[get("/user/<id>")]
fn get_user(id: u64) -> Option<Json<User>> {
    Some(Json(User {
        user_id: id,
        username: "bob".to_owned(),
        email: None,
    }))
}

// You can tag your routes to group them together
#[openapi(tag = "Users")]
#[post("/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}

// You can skip routes that you don't want to include in the openapi doc
#[openapi(skip)]
#[get("/hidden")]
fn hidden() -> Json<&'static str> {
    Json("Hidden from swagger!")
}

pub fn make_rocket() -> rocket::Rocket {
    rocket::build()
        // routes_with_openapi![...] will host the openapi document at openapi.json
        .mount(
            "/",
            routes_with_openapi![get_user, hidden],
        )
        // You can optionally host swagger-ui too
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
```

## TODO
- Tests
- Documentation
- Benchmark/optimise memory usage and allocations
  - Note to self: https://crates.io/crates/graphannis-malloc_size_of looks useful
- Implement `OpenApiFrom___`/`OpenApiResponder` for more rocket/rocket-contrib types
- Allow customizing openapi generation settings, e.g.
    - custom json schema generation settings
    - change path the document is hosted at

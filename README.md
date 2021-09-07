# okapi
okapi: [![Download](https://img.shields.io/crates/v/okapi)](https://crates.io/crates/okapi/)
[![API Docs](https://img.shields.io/badge/docs-okapi-blue)](https://docs.rs/okapi/latest/okapi/)

rocket-okapi: [![Download](https://img.shields.io/crates/v/rocket_okapi)](https://crates.io/crates/rocket_okapi)
[![API Docs](https://img.shields.io/badge/docs-rocket_okapi-blue)](https://docs.rs/rocket_okapi/latest/rocket_okapi/)

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Automated OpenAPI (AKA Swagger) document generation for Rust/Rocket projects.

Never have outdated documentation again.
Okapi will generate documentation for you while setting up the server.
It uses a combination of [Rust Doc comments](https://doc.rust-lang.org/reference/comments.html#doc-comments)
and programming logic to document your API.

The generated [OpenAPI][OpenAPI_3.0.0] files can then be used by various programs to
visualize the documentation. Rocket-okapi currently includes [RapiDoc][RapiDoc] and
[Swagger UI][Swagger_UI], but others can be used too.

Supported OpenAPI Spec: [3.0.0][OpenAPI_3.0.0]

Example of generated documentation using okapi:
- DF Storyteller: [RapiDoc](https://docs.dfstoryteller.com/rapidoc/), [Swagger UI](https://docs.dfstoryteller.com/swagger-ui/)
- ...[^1]

[OpenAPI_3.0.0]: https://spec.openapis.org/oas/v3.0.0
[RapiDoc]: https://mrin9.github.io/RapiDoc/
[Swagger_UI]: https://swagger.io/tools/swagger-ui/
[^1]: More examples will be added, please open an issue if you have a good example.

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
- [ ] Tests
- [ ] Documentation
- [ ] Benchmark/optimise memory usage and allocations
  - Note to self: https://crates.io/crates/graphannis-malloc_size_of looks useful
- [ ] Implement `OpenApiFrom___`/`OpenApiResponder` for more rocket/rocket-contrib types
- [ ] Allow customizing openapi generation settings, e.g.
    - [ ] custom json schema generation settings
    - [x] change path the document is hosted at

## License

This project is licensed under the [MIT license](LICENSE).

All contributions to this project will be similarly licensed.

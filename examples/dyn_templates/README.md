# Template example

Example using [Rockets template fairing](https://rocket.rs/v0.5/guide/responses/#templates).
This example uses the [`rocket_dyn_templates`](https://crates.io/crates/rocket_dyn_templates) crate.
It also uses `handlebars` but other templating engines work too.

Note the `"rocket_dyn_templates"` feature is enabled for the `rocket_okapi` crate in the toml file.

The openapi document will be hosted at `/openapi.json`, and the Swagger UI will be at `/swagger-ui`.
The RapiDoc UI will be at `/rapidoc`.
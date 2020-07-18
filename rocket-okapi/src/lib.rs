#![feature(specialization)]
#![forbid(missing_docs)]

//! This projects serves to enable automatic rendering of `openapi.json` files, and provides
//! facilities to also serve the documentation alongside your api.
//!
//! # Usage
//! First, add the following lines to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! rocket_okapi = "0.5"
//! schemars = "0.7"
//! okapi = { version = "0.5", features = ["derive_json_schema"] }
//! ```
//! To add documentation to a set of endpoints, a couple of steps are required. The request and
//! response types of the endpoint must implement `JsonSchema`. Secondly, the function must be
//! marked with `#[openapi]`. After that, you can simply replace `routes!` with
//! `routes_with_openapi!`. This will append an additional route to the resulting `Vec<Route>`,
//! which contains the `openapi.json` file that is required by swagger. Now that we have the json
//! file that we need, we can serve the swagger web interface at another endpoint, and we should be
//! able to load the example in the browser!
//! ### Example
//! ```rust, no_run
//! #![feature(decl_macro, proc_macro_hygiene)]
//!
//! use rocket::get;
//! use rocket_contrib::json::Json;
//! use rocket_okapi::{openapi, routes_with_openapi, JsonSchema};
//! use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
//!
//! #[derive(serde::Serialize, JsonSchema)]
//! struct Response {
//!     reply: String,
//! }
//!
//! #[openapi]
//! #[get("/")]
//! fn my_controller() -> Json<Response> {
//!     Json(Response {
//!         reply: "show me the docs!".to_string(),
//!     })
//! }
//!
//! fn get_docs() -> SwaggerUIConfig {
//!     use rocket_okapi::swagger_ui::UrlObject;
//!
//!     SwaggerUIConfig {
//!         url: "/my_resource/openapi.json".to_string(),
//!         urls: vec![UrlObject::new("My Resource", "/v1/company/openapi.json")],
//!         ..Default::default()
//!     }
//! }
//!
//! fn main() {
//!     rocket::ignite()
//!         .mount("/my_resource", routes_with_openapi![my_controller])
//!         .mount("/swagger", make_swagger_ui(&get_docs()))
//!         .launch();
//! }
//! ```

mod error;

/// Contains the `Generator` struct, which you can use to manually control the way a struct is
/// represented in the documentation.
pub mod gen;
/// Contains several `Rocket` `Handler`s, which are used for serving the json files and the swagger
/// interface.
pub mod handlers;
/// Contains an alternative UI for displaying the generated openapi spec, called
/// (Rapi Doc)[https://mrin9.github.io/RapiDoc/]
#[cfg(feature = "rapi")]
pub mod rapi_doc;
/// This module contains several traits that correspond to the `Rocket` traits pertaining to request
/// guards and responses
pub mod request;
/// Contains the trait `OpenApiResponder`, meaning that a response implementing this trait can be
/// documented.
pub mod response;
/// Contains then `OpenApiSettings` struct, which can be used to customise the behaviour of a
/// `Generator`.
pub mod settings;
/// Contains the functions and structs required to display the swagger web ui.
pub mod swagger_ui;
/// Assorted function that are used throughout the application.
pub mod util;

pub use error::*;
pub use rocket_okapi_codegen::*;
pub use schemars::JsonSchema;

/// Contains information about an endpoint.
pub struct OperationInfo {
    /// The path of the endpoint
    pub path: String,
    /// The HTTP Method of this endpoint.
    pub method: rocket::http::Method,
    /// Contains information to be showed in the documentation about this endpoint.
    pub operation: okapi::openapi3::Operation,
}

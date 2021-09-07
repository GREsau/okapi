#![forbid(unsafe_code)]
#![deny(clippy::all)]

//! This crate is used by [`rocket_okapi`](https://crates.io/crates/rocket_okapi)
//! for code generation. This crate includes the procedural macros like:
//! - `#[openapi]`: To generate the documentation for an endpoint/route.
//! - `routes_with_openapi![...]`: To generate and add the `openapi.json` route.
//! - `parse_openapi_routes![...]`: To generate and return a list of routes and the openapi spec.
//! - `create_openapi_spec![...]`: To generate and return the openapi spec.
//!
//! The last 3 macros have very similar behavior, but differ in what they return.
//! Here is a list of the marcos and what they return:
//! - `routes_with_openapi![...]`: `Vec<rocket::Route>` (adds route for `openapi.json`)
//! - `parse_openapi_routes![...]`: `(Vec<rocket::Route>, okapi::openapi3::OpenApi)`
//! - `create_openapi_spec![...]`: `okapi::openapi3::OpenApi`
//!

mod openapi_attr;
mod openapi_spec;
mod parse_routes;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

/// A proc macro to be used in tandem with one of `Rocket`'s endpoint macros. It requires that all
/// of the arguments of the route implement one of the traits in `rocket_okapi::request`, and that
/// the return type implements `OpenApiResponder`.
/// ### Example
/// ```rust,ignore
/// use rocket_okapi::openapi;
/// use rocket::get;
///
/// #[openapi]
/// #[get("/hello/<number>")]
/// fn hello_world(number: i32) -> String {
///     format!("Hello world number {}", number)
/// }
/// ```
#[proc_macro_attribute]
pub fn openapi(args: TokenStream, mut input: TokenStream) -> TokenStream {
    // We don't need to modify/replace the input TokenStream,
    // we just need to append to it.
    input.extend(openapi_attr::parse(args, input.clone()));
    input
}

/// A replacement macro for `rocket::routes`. The key differences are that this macro will add an
/// additional element to the resulting `Vec<rocket::Route>`, which serves a static file called
/// `openapi.json`. This file can then be used to display the routes in the Swagger/RapiDoc UI.
#[proc_macro]
pub fn routes_with_openapi(input: TokenStream) -> TokenStream {
    let settings = quote! {
        ::rocket_okapi::settings::OpenApiSettings::new()
    };
    let spec =
        openapi_spec::create_openapi_spec(input.clone()).unwrap_or_else(|e| e.to_compile_error());
    let routes = parse_routes::parse_routes(input, true).unwrap_or_else(|e| e.to_compile_error());
    (quote! {
        {
            let settings = #settings;
            let spec: ::okapi::openapi3::OpenApi = #spec(settings.clone());
            let routes: Vec<::rocket::Route> = #routes(spec, settings);
            routes
        }
    })
    .into()
}

/// A replacement macro for `rocket::routes`. This parses the routes and provides
/// a tuple with 2 parts `(Vec<rocket::Route>, OpenApi)`:
/// - `Vec<rocket::Route>`: A list of all the routes that `rocket::routes![]` would have provided.
/// - `OpenApi`: The `okapi::openapi3::OpenApi` spec for all the routes.
///
/// NOTE: This marco is different from `routes_with_openapi` in that it does not add
/// the `openapi.json` file to the list of routes. This is done so the `OpenApi` spec can be changed
/// before serving it.
#[proc_macro]
pub fn parse_openapi_routes(input: TokenStream) -> TokenStream {
    let settings = quote! {
        ::rocket_okapi::settings::OpenApiSettings::new()
    };
    let spec =
        openapi_spec::create_openapi_spec(input.clone()).unwrap_or_else(|e| e.to_compile_error());
    let routes = parse_routes::parse_routes(input, false).unwrap_or_else(|e| e.to_compile_error());
    (quote! {
        {
            let settings = #settings;
            let spec: ::okapi::openapi3::OpenApi = #spec(settings.clone());
            let routes: Vec<::rocket::Route> = #routes(spec.clone(), settings);
            (routes, spec)
        }
    })
    .into()
}

/// Generate `OpenApi` spec only, no parsing of routes.
/// This can be used in cases where you are only interested in openAPI spec, but not in the routes.
/// A use case could be inside of `build.rs` scripts or where you want to alter OpenAPI object
/// at runtime.
#[proc_macro]
pub fn create_openapi_spec(input: TokenStream) -> TokenStream {
    let settings = quote! {
        ::rocket_okapi::settings::OpenApiSettings::new()
    };
    let spec = openapi_spec::create_openapi_spec(input).unwrap_or_else(|e| e.to_compile_error());
    (quote! {
        {
            let settings = #settings;
            let spec: ::okapi::openapi3::OpenApi = #spec(settings);
            (spec)
        }
    })
    .into()
}

fn get_add_operation_fn_name(route_fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("okapi_add_operation_for_{}_", route_fn_name),
        route_fn_name.span(),
    )
}

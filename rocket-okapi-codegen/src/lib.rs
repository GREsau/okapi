#![forbid(unsafe_code)]
#![deny(clippy::all)]

//! This crate is used by [`rocket_okapi`](https://crates.io/crates/rocket_okapi)
//! for code generation. This crate includes the procedural macros like:
//! - `#[openapi]`: To generate the documentation for an endpoint/route.
//! - `openapi_routes![...]`: Returns a closure for generating routes.
//! - `openapi_spec![...]`: Returns a closure for generating OpenApi objects.
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

/// Generate and return a closure that can be used to generate the routes.
///
/// This closure take 2 arguments:
/// - `spec_opt`: `Option<::okapi::openapi3::OpenApi>`
/// - `settings`: `rocket_okapi::settings::OpenApiSettings`
///
/// It returns `Vec<::rocket::Route>`.
///
/// If `spec_opt` is set to `None` it will not add a route to serve the `openapi.json` file.
///
/// Example:
/// ```rust,ignore
/// let settings = rocket_okapi::settings::OpenApiSettings::new();
/// let spec = rocket_okapi::openapi_spec![get_message, post_message](settings.clone());
/// let routes = rocket_okapi::openapi_routes![get_message, post_message](Some(spec), settings);
/// ```
#[proc_macro]
pub fn openapi_routes(input: TokenStream) -> TokenStream {
    let routes = parse_routes::parse_routes(input).unwrap_or_else(|e| e.to_compile_error());
    (quote! {
        #routes
    })
    .into()
}

/// Generate and return a closure that can be used to generate the OpenAPI specification.
///
/// This closure take 1 argument:
/// - `settings`: `rocket_okapi::settings::OpenApiSettings`
///
/// It returns `okapi::openapi3::OpenApi`.
///
/// Example:
/// ```rust,ignore
/// let settings = rocket_okapi::settings::OpenApiSettings::new();
/// let spec = rocket_okapi::openapi_spec![get_message, post_message](settings);
/// ```
#[proc_macro]
pub fn openapi_spec(input: TokenStream) -> TokenStream {
    let spec = openapi_spec::create_openapi_spec(input).unwrap_or_else(|e| e.to_compile_error());
    (quote! {
        #spec
    })
    .into()
}

fn get_add_operation_fn_name(route_fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("okapi_add_operation_for_{}_", route_fn_name),
        route_fn_name.span(),
    )
}

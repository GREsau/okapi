#![forbid(unsafe_code)]
#![deny(clippy::all)]

//! This crate is used by [`rocket_okapi`](https://crates.io/crates/rocket_okapi)
//! for code generation. This crate includes the procedural macros like:
//! - `#[openapi]`: To generate the documentation for an endpoint/route.
//! - `openapi_routes![...]`: Returns a closure for generating routes.
//! - `openapi_spec![...]`: Returns a closure for generating OpenApi objects.
//! - `#[derive(OpenApiFromRequest)]`: Implement `OpenApiFromRequest` trait for a given struct.
//!

mod openapi_attr;
mod openapi_spec;
mod parse_routes;
mod responder_derive;

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
/// - `spec_opt`: `Option<rocket_okapi::okapi::openapi3::OpenApi>`
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
/// It returns `rocket_okapi::okapi::openapi3::OpenApi`.
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

/// Derive marco for the `OpenApiFromRequest` trait.
///
/// This derive trait is a very simple implementation for anything that does not
/// require any other special headers or parameters to be validated.
///
/// Use:
/// ```rust,ignore
/// use rocket_okapi::request::OpenApiFromRequest;
///
/// #[derive(OpenApiFromRequest)]
/// pub struct MyStructName;
/// ```
///
/// This code is equivalent to:
/// ```rust,ignore
/// use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
/// use rocket_okapi::gen::OpenApiGenerator;
///
/// pub struct MyStructName;
///
/// impl<'r> OpenApiFromRequest<'r> for MyStructName {
///     fn from_request_input(
///         _gen: &mut OpenApiGenerator,
///         _name: String,
///         _required: bool,
///     ) -> rocket_okapi::Result<RequestHeaderInput> {
///         Ok(RequestHeaderInput::None)
///     }
/// }
/// ```
#[proc_macro_derive(OpenApiFromRequest)]
pub fn open_api_from_request_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        impl<'r> rocket_okapi::request::OpenApiFromRequest<'r> for #name {
            fn from_request_input(
                _gen: &mut rocket_okapi::gen::OpenApiGenerator,
                _name: String,
                _required: bool,
            ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
                Ok(rocket_okapi::request::RequestHeaderInput::None)
            }
        }
    };
    gen.into()
}

/// TODO
#[proc_macro_derive(OpenApiResponder, attributes(responder))]
pub fn open_api_responder_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    match responder_derive::derive(ast) {
        Ok(v) => v.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn get_add_operation_fn_name(route_fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("okapi_add_operation_for_{}_", route_fn_name),
        route_fn_name.span(),
    )
}

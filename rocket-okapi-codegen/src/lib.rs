#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

mod openapi_attr;
mod routes_with_openapi;

use proc_macro::TokenStream;
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
///     format!("Hellow world number {}", number)
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
/// `openapi.json`. This file can then be used to display the routes in the swagger ui.
#[proc_macro]
pub fn routes_with_openapi(input: TokenStream) -> TokenStream {
    routes_with_openapi::parse(input)
}

#[proc_macro]
pub fn routes_with_openapi_with_settings(input: TokenStream) -> TokenStream {
    routes_with_openapi::parse_with_settings(input)
}

fn get_add_operation_fn_name(route_fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("okapi_add_operation_for_{}_", route_fn_name),
        route_fn_name.span(),
    )
}

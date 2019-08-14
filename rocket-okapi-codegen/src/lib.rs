#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

mod openapi_attr;
mod routes_with_openapi;

use proc_macro::TokenStream;
use syn::Ident;

#[proc_macro_attribute]
pub fn openapi(args: TokenStream, mut input: TokenStream) -> TokenStream {
    // We don't need to modify/replace the input TokenStream,
    // we just need to append to it.
    input.extend(openapi_attr::parse(args, input.clone()));
    input
}

#[proc_macro]
pub fn routes_with_openapi(input: TokenStream) -> TokenStream {
    routes_with_openapi::parse(input)
}

fn get_add_operation_fn_name(route_fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("_okapi_add_operation_for_{}_", route_fn_name),
        route_fn_name.span(),
    )
}

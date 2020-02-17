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
    input = preserve_span_information(input);

    // We don't need to modify/replace the input TokenStream,
    // we just need to append to it.
    input.extend(openapi_attr::parse(args, input.clone()));
    input
}

#[proc_macro]
pub fn routes_with_openapi(input: TokenStream) -> TokenStream {
    routes_with_openapi::parse(input)
}

fn preserve_span_information(input: TokenStream) -> TokenStream {
    // Outputting the input unmodified would cause its span information to be
    // lost when being consumed by other macros. But parsing it in then quoting
    // it back out causes span information to be preserved.
    // See https://github.com/GREsau/okapi/issues/12
    // and https://github.com/rust-lang/rust/issues/43081
    let parsed_input: syn::Item  = syn::parse(input).unwrap();
    quote!(#parsed_input).into()
}

fn get_add_operation_fn_name(route_fn_name: &Ident) -> Ident {
    Ident::new(
        &format!("okapi_add_operation_for_{}_", route_fn_name),
        route_fn_name.span(),
    )
}

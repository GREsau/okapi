use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, token::Comma, Path, Result};

/// Parses routes and returns a function that takes `OpenApi` and `OpenApiSettings` and
/// returns `Vec<rocket::Route>`.
/// It optionally adds the `openapi.json` route to the list of routes.
pub fn parse_routes(routes: TokenStream) -> Result<TokenStream2> {
    let paths = <Punctuated<Path, Comma>>::parse_terminated.parse(routes)?;
    // This returns a function so the spec does not have to be generated multiple times.
    Ok(quote! {
        |spec_opt: Option<::rocket_okapi::okapi::openapi3::OpenApi>, settings: &::rocket_okapi::settings::OpenApiSettings|
            -> Vec<::rocket::Route> {
                let mut routes = ::rocket::routes![#paths];
                if let Some(spec) = spec_opt {
                    routes.push(
                        ::rocket_okapi::handlers::OpenApiHandler::new(spec)
                            .into_route(&settings.json_path)
                    );
                }
                routes
        }
    })
}

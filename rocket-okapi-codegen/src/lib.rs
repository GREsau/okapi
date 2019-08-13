mod route_attr;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use rocket_http::{uri::Origin, MediaType, Method};
use std::collections::BTreeMap as Map;
use syn::{AttributeArgs, FnArg, Ident, ItemFn, ReturnType, Type, TypeTuple};

#[derive(Debug)]
struct Route {
    method: Method,
    origin: Origin<'static>,
    media_type: Option<MediaType>,
    data_param: Option<String>,
    //path_params: Vec<String>,
    //path_multi_param: Option<String>,
    //query_params: Vec<String>,
    //query_multi_param: Option<String>,
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct OkapiAttribute {
    pub skip: bool,
}

#[proc_macro_attribute]
pub fn openapi(args: TokenStream, mut input: TokenStream) -> TokenStream {
    // We don't need to modify/replace the input TokenStream,
    // we just need to append to it.
    input.extend(okapi_impl(args, input.clone()));
    input
}

fn okapi_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let okapi_attr = match OkapiAttribute::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    if okapi_attr.skip {
        return TokenStream::new();
    }

    match route_attr::parse_attrs(&input.attrs) {
        Ok(route) => create_route_operation_fn(input, route),
        Err(e) => e,
    }
}

fn create_route_operation_fn(route_fn: ItemFn, route: Route) -> TokenStream {
    let fn_decl = *route_fn.decl;
    let _arg_types = get_arg_types(fn_decl.inputs.into_iter());
    let return_type = match fn_decl.output {
        ReturnType::Type(_, ty) => *ty,
        ReturnType::Default => unit_type(),
    };

    let fn_name = Ident::new(
        &format!("_okapi_add_operation_for_{}_", route_fn.ident),
        Span::call_site(),
    );
    let path = route.origin.path().replace("<", "{").replace(">", "}");
    let method = Ident::new(&to_pascal_case_string(route.method), Span::call_site());

    TokenStream::from(quote! {
        fn #fn_name(
            gen: &mut ::rocket_okapi::gen::OpenApiGenerator,
            op_id: String,
        ) -> ::rocket_okapi::Result<()> {
            let responses = <#return_type as ::rocket_okapi::OpenApiResponses>::responses(gen)?;
            gen.add_operation(::rocket_okapi::OperationInfo {
                path: #path.to_owned(),
                method: ::rocket::http::Method::#method,
                operation: ::okapi::openapi3::Operation {
                    operation_id: Some(op_id),
                    responses,
                    ..Default::default()
                },
            });
            Ok(())
        }
    })
}

fn unit_type() -> Type {
    Type::Tuple(TypeTuple {
        paren_token: Default::default(),
        elems: Default::default(),
    })
}

fn to_pascal_case_string(method: Method) -> String {
    let (first_char, rest) = method.as_str().split_at(1);
    let first_char = first_char.to_ascii_uppercase();
    let rest = rest.to_ascii_lowercase();
    format!("{}{}", first_char, rest)
}

fn get_arg_types(args: impl Iterator<Item = FnArg>) -> Map<String, Type> {
    let mut result = Map::new();
    for arg in args {
        if let syn::FnArg::Captured(cap_arg) = arg {
            let name = cap_arg.pat.into_token_stream().to_string();
            let ty = cap_arg.ty;
            result.insert(name, ty);
        }
    }
    result
}

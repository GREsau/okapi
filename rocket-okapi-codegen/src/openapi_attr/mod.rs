mod doc_attr;
mod route_attr;

use crate::get_add_operation_fn_name;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use rocket_http::Method;
use std::collections::BTreeMap as Map;
use syn::{AttributeArgs, FnArg, Ident, ItemFn, ReturnType, Type, TypeTuple};

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct OpenApiAttribute {
    pub skip: bool,
}

pub fn parse(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let okapi_attr = match OpenApiAttribute::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    if okapi_attr.skip {
        return create_empty_route_operation_fn(input);
    }

    match route_attr::parse_attrs(&input.attrs) {
        Ok(route) => create_route_operation_fn(input, route),
        Err(e) => e,
    }
}

fn create_empty_route_operation_fn(route_fn: ItemFn) -> TokenStream {
    let fn_name = get_add_operation_fn_name(&route_fn.sig.ident);
    TokenStream::from(quote! {
        pub fn #fn_name(
            _gen: &mut ::rocket_okapi::gen::OpenApiGenerator,
            _op_id: String,
        ) -> ::rocket_okapi::Result<()> {
            Ok(())
        }
    })
}

fn create_route_operation_fn(route_fn: ItemFn, route: route_attr::Route) -> TokenStream {
    let arg_types = get_arg_types(route_fn.sig.inputs.into_iter());
    let return_type = match route_fn.sig.output {
        ReturnType::Type(_, ty) => *ty,
        ReturnType::Default => unit_type(),
    };
    let request_body = match &route.data_param {
        Some(arg) => {
            let ty = match arg_types.get(arg) {
                Some(ty) => ty,
                None => return quote! {
                    compile_error!(concat!("Could not find argument ", #arg, " matching data param."));
                }.into()
            };
            quote! {
                Some(<#ty as ::rocket_okapi::request::OpenApiFromData>::request_body(gen)?.into())
            }
        }
        None => quote! { None },
    };

    let mut params = Vec::new();
    // Path parameters: `/<id>/<name>`
    for arg in route.path_params() {
        let ty = match arg_types.get(arg) {
            Some(ty) => ty,
            None => return quote! {
                compile_error!(concat!("Could not find argument ", #arg, " matching path param."));
            }
            .into(),
        };
        params.push(quote! {
            <#ty as ::rocket_okapi::request::OpenApiFromParam>::path_parameter(gen, #arg.to_owned())?.into()
        })
    }
    // Query parameters: `/?<id>&<name>`
    for arg in route.query_params() {
        let ty = match arg_types.get(arg) {
            Some(ty) => ty,
            None => return quote! {
                compile_error!(concat!("Could not find argument ", #arg, " matching query param."));
            }
            .into(),
        };
        params.push(quote! {
            <#ty as ::rocket_okapi::request::OpenApiFromFormValue>::query_parameter(gen, #arg.to_owned(), true)?.into()
        })
    }
    let mut params_nested_list = Vec::new();
    // Multi Query parameters: `/?<param..>`
    for arg in route.query_multi_params() {
        let ty = match arg_types.get(arg) {
            Some(ty) => ty,
            None => return quote! {
                compile_error!(concat!("Could not find argument ", #arg, " matching query multi param."));
            }.into(),
        };
        params_nested_list.push(quote! {
            <#ty as ::rocket_okapi::request::OpenApiFromQuery>::query_multi_parameter(gen, #arg.to_owned(), true)?.into()
        })
    }

    let fn_name = get_add_operation_fn_name(&route_fn.sig.ident);
    let path = route
        .origin
        .path()
        .as_str()
        .replace("<", "{")
        .replace(">", "}");
    let method = Ident::new(&to_pascal_case_string(route.method), Span::call_site());
    let (title, desc) = doc_attr::get_title_and_desc_from_doc(&route_fn.attrs);
    let title = match title {
        Some(x) => quote!(Some(#x.to_owned())),
        None => quote!(None),
    };
    let desc = match desc {
        Some(x) => quote!(Some(#x.to_owned())),
        None => quote!(None),
    };

    TokenStream::from(quote! {
        pub fn #fn_name(
            gen: &mut ::rocket_okapi::gen::OpenApiGenerator,
            op_id: String,
        ) -> ::rocket_okapi::Result<()> {
            let responses = <#return_type as ::rocket_okapi::response::OpenApiResponder>::responses(gen)?;
            let request_body = #request_body;
            let mut parameters: Vec<::okapi::openapi3::RefOr<::okapi::openapi3::Parameter>> = vec![#(#params),*];
            // add nested lists
            let parameters_nested_list: Vec<Vec<::okapi::openapi3::Parameter>> = vec![#(#params_nested_list),*];
            for inner_list in parameters_nested_list{
                for item in inner_list{
                    // convert every item from `Parameter` to `RefOr<Parameter>``
                    parameters.push(item.into());
                }
            }
            gen.add_operation(::rocket_okapi::OperationInfo {
                path: #path.to_owned(),
                method: ::rocket::http::Method::#method,
                operation: ::okapi::openapi3::Operation {
                    operation_id: Some(op_id),
                    responses,
                    request_body,
                    parameters,
                    summary: #title,
                    description: #desc,
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
        if let syn::FnArg::Typed(arg) = arg {
            if let syn::Pat::Ident(ident) = *arg.pat {
                // Use only identifier name as key, so lookups succeed even if argument is mutable
                let name = ident.ident.into_token_stream().to_string();
                let ty = *arg.ty;
                result.insert(name, ty);
            }
        }
    }
    result
}

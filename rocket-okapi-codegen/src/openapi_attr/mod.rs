mod doc_attr;
mod route_attr;

use crate::get_add_operation_fn_name;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use rocket_http::Method;
use std::collections::BTreeMap as Map;
use syn::{parse_macro_input, AttributeArgs, FnArg, Ident, ItemFn, ReturnType, Type, TypeTuple};

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct OpenApiAttribute {
    pub skip: bool,

    #[darling(multiple, rename = "tag")]
    pub tags: Vec<String>,
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
        Ok(route) => create_route_operation_fn(input, route, okapi_attr.tags),
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

fn create_route_operation_fn(
    route_fn: ItemFn,
    route: route_attr::Route,
    tags: Vec<String>,
) -> TokenStream {
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

    // Parse Query Strings
    // https://rocket.rs/v0.5-rc/guide/requests/#query-strings
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
    // Multi Path parameters: `/<path..>`
    if let Some(arg) = route.path_multi_param() {
        let ty = match arg_types.get(arg) {
            Some(ty) => ty,
            None => return quote! {
                compile_error!(concat!("Could not find argument ", #arg, " matching multi path param."));
            }
            .into(),
        };
        params.push(quote! {
            <#ty as ::rocket_okapi::request::OpenApiFromSegments>::path_multi_parameter(gen, #arg.to_owned())?.into()
        })
    }
    let mut params_nested_list = Vec::new();
    // Query parameters: `/?<id>&<name>`
    for arg in route.query_params() {
        let ty = match arg_types.get(arg) {
            Some(ty) => ty,
            None => return quote! {
                compile_error!(concat!("Could not find argument ", #arg, " matching query param."));
            }
            .into(),
        };
        params_nested_list.push(quote! {
            <#ty as ::rocket_okapi::request::OpenApiFromForm>::form_multi_parameter(gen, #arg.to_owned(), true)?.into()
        })
    }
    // Multi Query parameters: `/?<param..>`
    for arg in route.query_multi_params() {
        let ty = match arg_types.get(arg) {
            Some(ty) => ty,
            None => return quote! {
                compile_error!(concat!("Could not find argument ", #arg, " matching multi query param."));
            }.into(),
        };
        params_nested_list.push(quote! {
            <#ty as ::rocket_okapi::request::OpenApiFromForm>::form_multi_parameter(gen, #arg.to_owned(), true)?.into()
        })
    }

    // Request quards, checks if the items are not found in the rocket route parameters, if that is the
    // case, we assume they are request guards
    let mut responses = Vec::new();
    responses.push(quote! {
      <#return_type as ::rocket_okapi::response::OpenApiResponder>::responses(gen)?
    });

    let data_param_arg = route.data_param.clone().unwrap_or_else(|| String::new());
    for arg_type in arg_types {
        let ty = arg_type.1;
        let arg = arg_type.0;

        // If the items are not found in the list of path/query parameters, assume the item is a request
        // guard and let them add to the openapi specification from the trait OpenApiFromRequest
        // Request guards can add or define their own responses, and can thus add to the possible
        // responses from an API
        if route
            .path_params()
            .find(|item| arg == item.to_string())
            .is_none()
            // Verify it is not in query parameters
            && route
                .query_params()
                .find(|item| arg == item.to_string())
                .is_none()
            && data_param_arg != arg
        {
            // println!("assuming request guard for: {:?}", arg);
            params.push(quote! {
                <#ty as ::rocket_okapi::request::OpenApiFromRequest>::request_input(gen, #arg.to_owned())?.into()
            });
            //TODO: implement that RequestGuards can specify the different types of responses

            // Create a response for this one
            // responses.push(quote! {
            //   <#ty as ::rocket_okapi::response::OpenApiResponder>::responses(gen)?
            // })
        }
    }

    let fn_name = get_add_operation_fn_name(&route_fn.sig.ident);
    let path = route
        .origin
        .path()
        .as_str()
        .replace("<", "{")
        .replace("..>", "}")
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

    let tags = tags
        .into_iter()
        .map(|tag| quote!(#tag.to_owned()))
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        pub fn #fn_name(
            gen: &mut ::rocket_okapi::gen::OpenApiGenerator,
            op_id: String,
        ) -> ::rocket_okapi::Result<()> {
            let responses = <#return_type as ::rocket_okapi::response::OpenApiResponder>::responses(gen)?;
            let request_body = #request_body;

            //###############
            use ::rocket_okapi::request::RequestHeaderInput;
            use ::okapi::openapi3::Parameter;
            use ::okapi::openapi3::RefOr;

            let request_inputs: Vec<RequestHeaderInput> = vec![#(#params),*];

            let mut parameters: Vec<::okapi::openapi3::RefOr<Parameter>> = Vec::new();
            use std::collections::BTreeMap as Map;
            let mut security_schemes = Map::new();
            for inp in request_inputs {
                match inp {
                    RequestHeaderInput::Parameter(p) => {
                       parameters.push(p.into());
                    }
                    RequestHeaderInput::Security(s) => {
                        // Make sure to add the security scheme listing
                        security_schemes.insert(s.0.scheme_identifier.clone(), Vec::new());
                        // Add the scheme to components definition of openapi
                        gen.add_security_scheme(s.0.scheme_identifier.clone(), s.0.clone());
                    }
                    _ => {
                    }
                }
            }
            let security = if security_schemes.is_empty() {
                None
            } else {
                Some(vec![security_schemes])
            };

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
                    security,
                    tags: vec![#(#tags),*],
                    ..::okapi::openapi3::Operation::default()
                },
            });
            Ok(())
        }
    })
}

fn unit_type() -> Type {
    Type::Tuple(TypeTuple {
        paren_token: syn::token::Paren::default(),
        elems: syn::punctuated::Punctuated::default(),
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

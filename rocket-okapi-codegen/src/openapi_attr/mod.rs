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
use syn::{
    parse_macro_input, AttributeArgs, FnArg, GenericArgument, Ident, ItemFn, PathArguments,
    PathSegment, ReturnType, Type, TypeTuple,
};

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

/// Replace `EventStream<impl SOMETHING>`
/// with `EventStream`
fn type_replace_impl_trait(ty: Type) -> Type {
    if let Type::Path(type_path) = &ty {
        if let Some(path_segment) = type_path.path.segments.first() {
            if let PathArguments::AngleBracketed(generic_argument) = &path_segment.arguments {
                if let Some(generic_argument) = generic_argument.args.first() {
                    if let Some(result_type) =
                        _type_replace_impl_trait_generic_argument(generic_argument, path_segment)
                    {
                        return result_type;
                    }
                }
            }
        }
    }
    ty
}

/// Helper function for `type_replace_impl_trait` and should not be called from anywhere else.
fn _type_replace_impl_trait_generic_argument(
    gen_arg: &GenericArgument,
    path_segment: &PathSegment,
) -> Option<Type> {
    if let GenericArgument::Type(Type::ImplTrait(_)) = gen_arg {
        if path_segment.ident == "EventStream" {
            // Return special type, the type of stream does not matter as long as something is present
            return Some(Type::Verbatim(quote! {
                EventStream<rocket::futures::stream::Empty<rocket::response::stream::Event>>
            }));
        } else if path_segment.ident == "ByteStream" {
            // Return special type, the type of stream does not matter as long as something is present
            return Some(Type::Verbatim(quote! {
                ByteStream<rocket::futures::stream::Empty<Vec<u8>>>
            }));
        } else if path_segment.ident == "ReaderStream" {
            // Return special type, the type of stream does not matter as long as something is present
            return Some(Type::Verbatim(quote! {
                ReaderStream<rocket::futures::stream::Empty<File>>
            }));
        } else if path_segment.ident == "TextStream" {
            // Return special type, the type of stream does not matter as long as something is present
            return Some(Type::Verbatim(quote! {
                TextStream<rocket::futures::stream::Empty<String>>
            }));
        }
    }
    None
}

fn create_route_operation_fn(
    route_fn: ItemFn,
    route: route_attr::Route,
    tags: Vec<String>,
) -> TokenStream {
    let arg_types = get_arg_types(route_fn.sig.inputs.into_iter());
    let return_type = match route_fn.sig.output {
        ReturnType::Type(_, ty) => type_replace_impl_trait(*ty),
        ReturnType::Default => unit_type(),
    };

    // ----- Check route info -----

    // -- Parse Query Strings --
    // https://rocket.rs/v0.5-rc/guide/requests/#query-strings
    let mut params = Vec::new();
    let mut params_nested_list = Vec::new();
    let mut params_request_guards = Vec::new();
    let mut request_guard_responses = Vec::new();
    // Create a list of all the already used parameters.
    // This is later used to see what parameters are Request Guards. (aka left over)
    let mut params_names_used = Vec::new();
    // Path parameters: `/<id>/<name>`
    for arg in route.path_params() {
        let ty = match arg_types.get(arg) {
            Some(ty) => ty,
            None => return quote! {
                compile_error!(concat!("Could not find argument ", #arg, " matching path param."));
            }
            .into(),
        };
        params_names_used.push(arg.to_owned());
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
        params_names_used.push(arg.to_owned());
        params.push(quote! {
            <#ty as ::rocket_okapi::request::OpenApiFromSegments>::path_multi_parameter(gen, #arg.to_owned())?.into()
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
        params_names_used.push(arg.to_owned());
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
        params_names_used.push(arg.to_owned());
        params_nested_list.push(quote! {
            <#ty as ::rocket_okapi::request::OpenApiFromForm>::form_multi_parameter(gen, #arg.to_owned(), true)?.into()
        })
    }

    // -- Body Data --
    // https://rocket.rs/v0.5-rc/guide/requests/#body-data
    let request_body = match &route.data_param {
        Some(data_param) => {
            let ty = match arg_types.get(data_param) {
                Some(ty) => ty,
                None => return quote! {
                    compile_error!(concat!("Could not find argument ", #data_param, " matching data param."));
                }.into()
            };
            // Add parameter to list
            params_names_used.push(data_param.clone());
            quote! {
                Some(<#ty as ::rocket_okapi::request::OpenApiFromData>::request_body(gen)?.into())
            }
        }
        None => quote! { None },
    };

    // -- Request Guards --
    // https://rocket.rs/v0.5-rc/guide/requests/#request-guards
    // Request Guards is every that is not already used and thus not in `params_names_used`.
    for (arg, ty) in &arg_types {
        if !params_names_used.contains(arg) {
            params_names_used.push(arg.to_owned());
            params_request_guards.push(quote! {
                <#ty as ::rocket_okapi::request::OpenApiFromRequest>::from_request_input(gen, #arg.to_owned(), true)?.into()
            });
            request_guard_responses.push(quote! {
                <#ty as ::rocket_okapi::request::OpenApiFromRequest>::get_responses(gen)?.into()
            });
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
            let mut responses = <#return_type as ::rocket_okapi::response::OpenApiResponder>::responses(gen)?;
            // Add responses from Request Guards.
            let request_guard_responses = vec![#(#request_guard_responses),*];
            for request_guard_response in request_guard_responses {
                ::rocket_okapi::okapi::merge::merge_responses(&mut responses, &request_guard_response)?;
            }

            let request_body = #request_body;
            // Add the security scheme that are quired for all the routes.
            let mut security_requirements = Vec::new();

            // Combine all parameters from all sources
            // Add all from `path_params` and `path_multi_param`
            let mut parameters: Vec<::rocket_okapi::okapi::openapi3::RefOr<::rocket_okapi::okapi::openapi3::Parameter>> = vec![#(#params),*];
            // Add all from `query_params` and `query_multi_params`
            let parameters_nested_list: Vec<Vec<::rocket_okapi::okapi::openapi3::Parameter>> = vec![#(#params_nested_list),*];
            for inner_list in parameters_nested_list{
                for item in inner_list{
                    // convert every item from `Parameter` to `RefOr<Parameter>``
                    parameters.push(item.into());
                }
            }
            // Body Data does not add any parameters

            // Add all Request Guards
            let request_guards_route: Vec<::rocket_okapi::request::RequestHeaderInput> = vec![#(#params_request_guards),*];
            for request_guard_route in request_guards_route {
                use ::rocket_okapi::request::RequestHeaderInput;
                match request_guard_route {
                    // Add Parameters
                    RequestHeaderInput::Parameter(p) => {
                       parameters.push(p.into());
                    }
                    // Add Security Schemes, different section.
                    RequestHeaderInput::Security(name, schema, requirement) => {
                        // Add/replace the security scheme (global).
                        gen.add_security_scheme(name, schema);
                        // Add the security scheme that are quired for all the route.
                        security_requirements.push(requirement);
                    }
                    _ => {
                    }
                }
            }

            // Add `security` section if list is not empty
            let security = if security_requirements.is_empty() {
                None
            } else {
                Some(security_requirements)
            };
            // Add route/endpoint to OpenApi object.
            gen.add_operation(::rocket_okapi::OperationInfo {
                path: #path.to_owned(),
                method: ::rocket::http::Method::#method,
                operation: ::rocket_okapi::okapi::openapi3::Operation {
                    operation_id: Some(op_id),
                    responses,
                    request_body,
                    parameters,
                    summary: #title,
                    description: #desc,
                    security,
                    tags: vec![#(#tags),*],
                    ..Default::default()
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

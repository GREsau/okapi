use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use rocket_http::{ext::IntoOwned, uri::Origin, MediaType, Method};
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{Attribute, Meta, MetaList, NestedMeta};

#[derive(Debug)]
pub struct Route {
    pub method: Method,
    pub origin: Origin<'static>,
    #[allow(dead_code)]
    pub media_type: Option<MediaType>,
    pub data_param: Option<String>,
}

impl Route {
    pub fn path_params(&self) -> impl Iterator<Item = &str> {
        self.origin.path().segments().filter_map(|s| {
            if s.starts_with('<') && s.ends_with('>') && !s.ends_with("..>") {
                Some(&s[1..s.len() - 1])
            } else {
                None
            }
        })
    }

    pub fn path_multi_param(&self) -> Option<&str> {
        self.origin.path().segments().find_map(|s| {
            if s.starts_with('<') && s.ends_with("..>") {
                Some(&s[1..s.len() - 3])
            } else {
                None
            }
        })
    }

    pub fn query_params(&self) -> impl Iterator<Item = &str> {
        let mut query_params: Vec<&str> = vec![];
        if let Some(query) = self.origin.query() {
            query_params = query.as_str().split('&').collect();
            query_params = query_params
                .into_iter()
                .filter_map(|s| {
                    if s.starts_with('<') && s.ends_with('>') && !s.ends_with("..>") {
                        Some(&s[1..s.len() - 1])
                    } else {
                        None
                    }
                })
                .collect();
        }
        query_params.into_iter()
    }

    pub fn query_multi_params(&self) -> impl Iterator<Item = &str> {
        let mut query_params: Vec<&str> = vec![];
        if let Some(query) = self.origin.query() {
            query_params = query.as_str().split('&').collect();
            query_params = query_params
                .into_iter()
                .filter_map(|s| {
                    if s.starts_with('<') && s.ends_with("..>") {
                        Some(&s[1..s.len() - 3])
                    } else {
                        None
                    }
                })
                .collect();
        }
        query_params.into_iter()
    }
}

#[derive(Debug)]
struct OriginMeta(Origin<'static>);
#[derive(Debug)]
struct MediaTypeMeta(MediaType);
#[derive(Debug)]
struct MethodMeta(Method);

impl FromMeta for OriginMeta {
    fn from_string(value: &str) -> Result<Self, Error> {
        match Origin::parse_route(value) {
            Ok(o) => Ok(OriginMeta(o.into_owned())),
            Err(e) => Err(Error::unsupported_format(&e.to_string())),
        }
    }
}

impl FromMeta for MediaTypeMeta {
    fn from_string(value: &str) -> Result<Self, Error> {
        match MediaType::parse_flexible(value) {
            Some(m) => Ok(MediaTypeMeta(m)),
            None => Err(Error::unsupported_format(&format!(
                "Unknown media type: '{}'",
                value
            ))),
        }
    }
}

impl FromMeta for MethodMeta {
    fn from_string(value: &str) -> Result<Self, Error> {
        match Method::from_str(value) {
            Ok(m) => Ok(MethodMeta(m)),
            Err(e) => Err(Error::unsupported_format(&format!(
                "Unknown HTTP method: '{}'. Error: {}",
                value, e
            ))),
        }
    }
}

#[derive(Debug, FromMeta)]
#[darling(allow_unknown_fields)]
struct RouteAttributeNamedMeta {
    path: OriginMeta,
    #[darling(default)]
    format: Option<MediaTypeMeta>,
    #[darling(default)]
    data: Option<String>,
}

#[derive(Debug, FromMeta)]
#[darling(allow_unknown_fields)]
struct MethodRouteAttributeNamedMeta {
    #[darling(default)]
    format: Option<MediaTypeMeta>,
    #[darling(default)]
    data: Option<String>,
}

fn parse_route_attr(args: &[NestedMeta]) -> Result<Route, Error> {
    if args.is_empty() {
        return Err(Error::too_few_items(1));
    }
    let method = MethodMeta::from_nested_meta(&args[0])?;
    let named = RouteAttributeNamedMeta::from_list(&args[1..])?;
    Ok(Route {
        method: method.0,
        origin: named.path.0,
        media_type: named.format.map(|x| x.0),
        data_param: named.data.map(trim_angle_brackers),
    })
}

fn parse_method_route_attr(method: Method, args: &[NestedMeta]) -> Result<Route, Error> {
    if args.is_empty() {
        return Err(Error::too_few_items(1));
    }
    let origin = OriginMeta::from_nested_meta(&args[0])?;
    let named = MethodRouteAttributeNamedMeta::from_list(&args[1..])?;
    Ok(Route {
        method,
        origin: origin.0,
        media_type: named.format.map(|x| x.0),
        data_param: named.data.map(trim_angle_brackers),
    })
}

fn trim_angle_brackers(mut s: String) -> String {
    if s.starts_with('<') && s.ends_with('>') {
        s.pop();
        s.remove(0);
    }
    s
}

fn parse_attr(name: &str, args: &[NestedMeta]) -> Result<Route, Error> {
    match Method::from_str(name) {
        Ok(method) => parse_method_route_attr(method, args),
        Err(_) => parse_route_attr(args),
    }
}

fn is_route_attribute(a: &Attribute) -> bool {
    a.path.is_ident("get")
        || a.path.is_ident("put")
        || a.path.is_ident("post")
        || a.path.is_ident("delete")
        || a.path.is_ident("options")
        || a.path.is_ident("head")
        || a.path.is_ident("trace")
        || a.path.is_ident("connect")
        || a.path.is_ident("patch")
        || a.path.is_ident("route")
}

fn to_name_and_args(attr: &Attribute) -> Option<(String, Vec<NestedMeta>)> {
    match attr.parse_meta() {
        Ok(Meta::List(MetaList { path, nested, .. })) => path
            .get_ident()
            .map(|name| (name.to_string(), nested.into_iter().collect())),
        _ => None,
    }
}

pub(crate) fn parse_attrs<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
) -> Result<Route, TokenStream> {
    match attrs.into_iter().find(|a| is_route_attribute(a)) {
        Some(attr) => {
            let span = attr.span();
            let (name, args) = to_name_and_args(attr)
                .ok_or_else(|| TokenStream::from(quote_spanned! {span=>
                    compile_error!("Malformed route attribute");
                }))?;

            parse_attr(&name, &args)
                .map_err(|e| e.with_span(&attr).write_errors().into())
        }
        None => Err(quote! {
                compile_error!("Could not find Rocket route attribute. Ensure the #[openapi] attribute is placed *before* the Rocket route attribute.");
            }.into()),
    }
}

use crate::Route;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use rocket_http::{ext::IntoOwned, uri::Origin, MediaType, Method};
use std::str::FromStr;
use syn::AttributeArgs;

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
            Err(()) => Err(Error::unsupported_format(&format!(
                "Unknown HTTP method: '{}'",
                value
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

fn parse_route_attr(args: &AttributeArgs) -> Result<Route, Error> {
    if args.is_empty() {
        return Err(Error::too_few_items(1));
    }
    let method = MethodMeta::from_nested_meta(&args[0])?;
    let named = RouteAttributeNamedMeta::from_list(&args[1..])?;
    Ok(Route {
        method: method.0,
        origin: named.path.0,
        media_type: named.format.map(|x| x.0),
        data_param: named.data,
    })
}

fn parse_method_route_attr(method: Method, args: &AttributeArgs) -> Result<Route, Error> {
    if args.is_empty() {
        return Err(Error::too_few_items(1));
    }
    let origin = OriginMeta::from_nested_meta(&args[0])?;
    let named = MethodRouteAttributeNamedMeta::from_list(&args[1..])?;
    Ok(Route {
        method: method,
        origin: origin.0,
        media_type: named.format.map(|x| x.0),
        data_param: named.data,
    })
}

pub(crate) fn parse_attr(name: &str, args: &AttributeArgs) -> Result<Route, TokenStream> {
    let parsed = match Method::from_str(name) {
        Ok(method) => parse_method_route_attr(method, args),
        Err(()) => parse_route_attr(args),
    };
    parsed.map_err(|e| e.write_errors().into())
}

mod route_attr;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use rocket_http::{uri::Origin, MediaType, Method};
use syn::{AttributeArgs, ItemFn};

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
pub fn okapi(args: TokenStream, mut input: TokenStream) -> TokenStream {
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

    let _args = match route_attr::parse_attrs(input.attrs) {
        Ok(v) => v,
        Err(e) => {
            return e;
        }
    };

    // do things with `args`
    unimplemented!()
}

mod attr;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

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

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let _input = parse_macro_input!(input as ItemFn);

    let _args = match attr::parse_attr("get", &attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e;
        }
    };

    // do things with `args`
    unimplemented!()
}

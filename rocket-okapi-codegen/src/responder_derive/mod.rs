mod attribute;
mod utils;

use attribute::ResponseAttribute;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Attribute, DeriveInput, Error, Field, Fields, Result};
use utils::{parse_attribute, parse_doc};

pub(crate) fn derive(input: DeriveInput) -> Result<TokenStream> {
    let variants: Vec<(Vec<Attribute>, Fields)> = match input.data {
        syn::Data::Struct(s) => vec![(input.attrs.clone(), s.fields)],
        syn::Data::Enum(e) => e
            .variants
            .into_iter()
            .map(|v| (v.attrs, v.fields))
            .collect(),
        syn::Data::Union(_) => {
            return Err(Error::new(input.span(), "unions are not supported"));
        }
    };

    let attrs = input.attrs;
    let ident = input.ident;
    let generics = input.generics;

    let responses = variants
        .into_iter()
        .filter_map(|v| variant_to_responses(&attrs, v).transpose())
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        impl<#generics> ::rocket_okapi::response::OpenApiResponderInner for #ident {
            fn responses(
                gen: &mut ::rocket_okapi::gen::OpenApiGenerator,
            ) -> ::rocket_okapi::Result<::rocket_okapi::okapi::openapi3::Responses> {
                let mut responses = ::rocket_okapi::okapi::openapi3::Responses::default();
                #(
                    responses.responses.extend({ #responses }.responses);
                )*
                Ok(responses)
            }
        }
    })
}

fn variant_to_responses(
    global_attrs: &[Attribute],
    (attrs, fields): (Vec<Attribute>, Fields),
) -> Result<Option<TokenStream>> {
    let ResponseAttribute {
        ignore,
        status,
        content_type,
    } = parse_response_attribute(&attrs)?;

    if ignore {
        return Ok(None);
    }

    let Some(Field {
        attrs: field_attrs,
        ty: field_type,
        ..
    }) = first_field(fields)?
    else {
        return Ok(None);
    };

    let set_status = (status.0 != rocket_http::Status::Ok)
        .then(|| quote! { ::rocket_okapi::util::set_status_code(&mut r, #status)?; });

    let set_content_type =
        content_type.map(|ct| quote! { ::rocket_okapi::util::set_content_type(&mut r, #ct)?; });

    let set_description = parse_doc(&field_attrs)
        .or_else(|| parse_doc(&attrs))
        .or_else(|| parse_doc(global_attrs))
        .map(|doc| quote! { ::rocket_okapi::util::set_description(&mut r, #doc)?; });

    let responses = quote! {
        let mut r = <#field_type as ::rocket_okapi::response::OpenApiResponderInner>::responses(gen)?;
        #set_status
        #set_content_type
        #set_description
        r
    };

    Ok(Some(responses))
}

fn first_field(fields: Fields) -> Result<Option<Field>> {
    for field in fields {
        let ResponseAttribute { ignore, .. } = parse_response_attribute(&field.attrs)?;
        if !ignore {
            return Ok(Some(field));
        }
    }

    Ok(None)
}

#[inline(always)]
fn parse_response_attribute(attrs: &[Attribute]) -> Result<ResponseAttribute> {
    parse_attribute(&attrs, "response").map(Option::unwrap_or_default)
}

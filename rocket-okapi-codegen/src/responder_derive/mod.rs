mod response_attr;

use crate::doc_attr::get_doc;
use darling::{FromMeta, Result};
use proc_macro2::TokenStream;
use quote::quote;
use response_attr::ResponseAttribute;
use syn::{parse_quote, Attribute, DeriveInput, Fields, GenericParam, Generics};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let responses_variants: Vec<(Vec<Attribute>, Fields)> = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            [(input.attrs.clone(), fields)].to_vec()
        }
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            variants.into_iter().map(|v| (v.attrs, v.fields)).collect()
        }
        syn::Data::Union(_) => {
            return Err(darling::Error::custom("unions are not supported").with_span(&input));
        }
    };

    let name = input.ident;

    let generics = add_trait_bound(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let responses = responses_variants
        .into_iter()
        .filter_map(|(variant_attrs, variant_fields)| {
            variant_to_responses(&input.attrs, variant_attrs, variant_fields).transpose()
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        impl #impl_generics ::rocket_okapi::response::OpenApiResponderInner for #name #ty_generics #where_clause {
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

fn add_trait_bound(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(param) = param {
            param.bounds.push(parse_quote!(
                ::rocket_okapi::response::OpenApiResponderInner
            ));
        }
    }
    generics
}

fn variant_to_responses(
    entity_attrs: &[Attribute],
    attrs: Vec<Attribute>,
    fields: Fields,
) -> Result<Option<TokenStream>> {
    let response_attribute = get_response_attr_or_default(&attrs)?;

    let field = fields
        .into_iter()
        .next()
        .ok_or(darling::Error::custom("need at least one field"))?;
    let response_type = &field.ty;

    let status = response_attribute.status;
    let set_status =
        status.map(|status| quote! { ::rocket_okapi::util::set_status_code(&mut r, #status)?; });

    let content_type = response_attribute.content_type;
    let set_content_type =
        content_type.map(|ct| quote! { ::rocket_okapi::util::set_content_type(&mut r, #ct)?; });

    let description = get_doc(&field.attrs)
        .or_else(|| get_doc(&attrs))
        .or_else(|| get_doc(entity_attrs));
    let set_description =
        description.map(|doc| quote! { ::rocket_okapi::util::set_description(&mut r, #doc)?; });

    Ok(Some(quote! {
        let mut r = <#response_type as ::rocket_okapi::response::OpenApiResponderInner>::responses(gen)?;
        #set_status
        #set_content_type
        #set_description
        r
    }))
}

fn get_response_attr_or_default(attrs: &[Attribute]) -> Result<ResponseAttribute> {
    let mut attrs = attrs.iter().filter(|a| a.path.is_ident("response"));

    let Some(attr) = attrs.next() else {
        return Ok(ResponseAttribute::default());
    };

    if let Some(second_attr) = attrs.next() {
        return Err(
            darling::Error::custom("`response` attribute may be specified only once")
                .with_span(second_attr),
        );
    }

    let meta = attr.parse_meta()?;
    ResponseAttribute::from_meta(&meta)
}

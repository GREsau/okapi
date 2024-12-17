use quote::ToTokens;
use syn::{parse::Parse, Error, LitInt, LitStr, Token};

mod kw {
    syn::custom_keyword!(ignore);
    syn::custom_keyword!(status);
    syn::custom_keyword!(content_type);
}

#[derive(Default)]
pub(crate) struct ResponseAttribute {
    pub ignore: bool,
    pub status: Status,
    pub content_type: Option<ContentType>,
}

#[derive(Default)]
#[repr(transparent)]
pub(crate) struct Status(pub rocket_http::Status);

impl ToTokens for Status {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.code.to_tokens(tokens);
    }
}

#[derive(Default)]
#[repr(transparent)]
pub(crate) struct ContentType(pub rocket_http::ContentType);

impl ToTokens for ContentType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_string().to_tokens(tokens);
    }
}

impl Parse for ResponseAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = Self::default();
        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::ignore) {
                input.parse::<kw::ignore>()?;
                out.ignore = true;
            } else if lookahead.peek(kw::status) {
                input.parse::<kw::status>()?;
                input.parse::<Token![=]>()?;
                let code = input.parse::<LitInt>()?.base10_parse::<u16>()?;
                out.status = Status(rocket_http::Status::new(code));
            } else if lookahead.peek(kw::content_type) {
                input.parse::<kw::content_type>()?;
                input.parse::<Token![=]>()?;
                let value = input.parse::<LitStr>()?;
                let content_type = value
                    .value()
                    .parse()
                    .map_err(|err| Error::new(value.span(), err))?;
                out.content_type = Some(ContentType(content_type));
            } else if input.is_empty() {
                break;
            } else {
                return Err(lookahead.error());
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        Ok(out)
    }
}

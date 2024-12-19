use darling::FromMeta;
use quote::ToTokens;

/// This structure documents all the properties that can be used in
/// the `#[response]` attribute. For example: `#[response(status = 404)]`
#[derive(Default, FromMeta)]
#[darling(default)]
pub struct ResponseAttribute {
    /// Status code of the response. By default, depends on the field type
    pub status: Option<Status>,

    /// Content type of the response. By default, depends on the field type
    pub content_type: Option<ContentType>,
}

/// Derive macro wrapper for the Rocket's Status type.
/// Implements [`darling::FromMeta`] and [`quote::ToTokens`].
pub struct Status(pub rocket_http::Status);

impl Default for Status {
    fn default() -> Self {
        Self(rocket_http::Status::Ok)
    }
}

impl FromMeta for Status {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        let syn::Lit::Int(int) = value else {
            return Err(darling::Error::unexpected_lit_type(value));
        };

        let code = int.base10_parse::<u16>()?;
        if code < 100 || code >= 600 {
            return Err(darling::Error::custom("status must be in range [100, 599]").with_span(int));
        }

        Ok(Self(rocket_http::Status::new(code)))
    }
}

impl ToTokens for Status {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.code.to_tokens(tokens);
    }
}

/// Derive macro wrapper for the Rocket's ContentType type.
/// Implements [`darling::FromMeta`] and [`quote::ToTokens`].
pub struct ContentType(pub rocket_http::ContentType);

impl FromMeta for ContentType {
    fn from_string(value: &str) -> darling::Result<Self> {
        rocket_http::ContentType::parse_flexible(value)
            .map(Self)
            .ok_or_else(|| darling::Error::unsupported_format(value))
    }
}

impl ToTokens for ContentType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_string().to_tokens(tokens);
    }
}

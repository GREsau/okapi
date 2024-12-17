use syn::{parse::Parse, spanned::Spanned, Attribute, Error, Expr, Ident, Lit, Meta, Result};

pub(crate) fn parse_doc(attrs: &[Attribute]) -> Option<String> {
    let doc = attrs
        .iter()
        .filter(|a| a.path.is_ident("doc"))
        .filter_map(|a| {
            if let Meta::NameValue(nv) = &a.parse_meta().ok()? {
                if let Lit::Str(str) = &nv.lit {
                    return Some(str.value().trim().to_owned());
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join(" ");

    (!doc.is_empty()).then_some(doc)
}

pub(crate) fn parse_attribute<A: Parse, I>(attrs: &[Attribute], ident: I) -> Result<Option<A>>
where
    Ident: PartialEq<I>,
{
    let attrs = attrs
        .iter()
        .filter(|a| a.path.is_ident(&ident))
        .collect::<Vec<_>>();

    if attrs.len() > 1 {
        return Err(Error::new(
            attrs[1].span(),
            "this attribute may be specified only once",
        ));
    } else if attrs.is_empty() {
        return Ok(None);
    }

    let attr = attrs[0];
    let attr = attr.parse_args::<A>()?;

    Ok(Some(attr))
}

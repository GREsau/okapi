use crate::{OpenApiError, Result};
use okapi::openapi3::*;

pub fn add_response(responses: &mut Responses, status: u16) -> &mut RefOr<Response> {
    responses
        .responses
        .entry(status.to_string())
        .or_insert_with(|| Response::default().into())
}

pub fn add_schema_response(
    responses: &mut Responses,
    status: u16,
    content_type: impl ToString,
    schema: RefOr<SchemaObject>,
) -> Result<()> {
    let media = MediaType {
        schema: Some(schema),
        ..Default::default()
    };
    let response = match add_response(responses, status) {
        RefOr::Ref(_) => {
            return Err(OpenApiError::new(
                "Altering Ref responses is not supported.".to_owned(),
            ))
        }
        RefOr::Object(o) => o,
    };
    // FIXME these clones shouldn't be necessary
    response
        .content
        .entry(content_type.to_string())
        .and_modify(|mt| *mt = accept_either_media_type(mt.clone(), media.clone()))
        .or_insert(media);
    Ok(())
}

fn accept_either_media_type(mt1: MediaType, mt2: MediaType) -> MediaType {
    fn extend<A, E: Extend<A>>(mut a: E, b: impl IntoIterator<Item = A>) -> E {
        a.extend(b);
        a
    }

    MediaType {
        schema: accept_either_schema(mt1.schema, mt2.schema),
        example: mt1.example.or(mt2.example),
        examples: match (mt1.examples, mt2.examples) {
            (Some(e1), Some(e2)) => Some(extend(e1, e2)),
            (Some(e), None) | (None, Some(e)) => Some(e),
            (None, None) => None,
        },
        encoding: extend(mt1.encoding, mt2.encoding),
        extensions: extend(mt1.extensions, mt2.extensions),
    }
}

fn accept_either_schema(
    s1: Option<RefOr<SchemaObject>>,
    s2: Option<RefOr<SchemaObject>>,
) -> Option<RefOr<SchemaObject>> {
    let (s1, s2) = match (s1, s2) {
        (Some(s1), Some(s2)) => (s1, s2),
        (Some(s), None) | (None, Some(s)) => (s, SchemaObject::default().into()),
        (None, None) => return None,
    };
    Some(
        SchemaObject {
            any_of: Some(vec![s1.into(), s2.into()]),
            ..Default::default()
        }
        .into(),
    )
}

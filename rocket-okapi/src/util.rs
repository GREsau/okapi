use okapi::openapi3::*;
use okapi::Map;

pub fn schema_response(
    status: u16,
    media_type: impl ToString,
    schema: RefOr<SchemaObject>,
) -> Responses {
    Responses {
        responses: {
            let mut map = Map::new();
            map.insert(
                status.to_string(),
                Response {
                    content: {
                        let mut map = Map::new();
                        map.insert(
                            media_type.to_string(),
                            MediaType {
                                schema: Some(schema),
                                ..Default::default()
                            },
                        );
                        map
                    },
                    ..Default::default()
                }
                .into(),
            );
            map
        },
        ..Default::default()
    }
}

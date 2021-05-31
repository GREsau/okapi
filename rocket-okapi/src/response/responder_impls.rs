use super::OpenApiResponderInner;
use crate::{gen::OpenApiGenerator, util::*};
use okapi::openapi3::Responses;
use rocket::fs::NamedFile;
use rocket::serde::json::{Json, Value};
use schemars::JsonSchema;
use serde::Serialize;
use std::fmt::Debug;
use std::result::Result as StdResult;

type Result = crate::Result<Responses>;

impl<T: Serialize + JsonSchema + Send> OpenApiResponderInner for Json<T> {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<T>();
        add_schema_response(&mut responses, 200, "application/json", schema)?;
        Ok(responses)
    }
}

impl OpenApiResponderInner for Value {
    fn responses(_gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        let schema = schemars::schema::Schema::Bool(true);
        add_schema_response(&mut responses, 200, "application/json", schema.into())?;
        Ok(responses)
    }
}

impl OpenApiResponderInner for String {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<String>();
        add_schema_response(&mut responses, 200, "text/plain", schema)?;
        Ok(responses)
    }
}

impl OpenApiResponderInner for &str {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <String>::responses(gen)
    }
}

impl OpenApiResponderInner for Vec<u8> {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        add_content_response(
            &mut responses,
            200,
            "application/octet-stream",
            Default::default(),
        )?;
        Ok(responses)
    }
}

impl OpenApiResponderInner for &[u8] {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <Vec<u8>>::responses(gen)
    }
}

impl OpenApiResponderInner for () {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        ensure_status_code_exists(&mut responses, 200);
        Ok(responses)
    }
}

impl<T: OpenApiResponderInner> OpenApiResponderInner for Option<T> {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = T::responses(gen)?;
        ensure_status_code_exists(&mut responses, 404);
        Ok(responses)
    }
}

impl OpenApiResponderInner for NamedFile {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <Vec<u8>>::responses(gen)
    }
}

macro_rules! status_responder {
    ($responder: ident, $status: literal) => {
        impl<T> OpenApiResponderInner for rocket::response::status::$responder<T>
        where
            T: OpenApiResponderInner + Send,
        {
            fn responses(gen: &mut OpenApiGenerator) -> Result {
                let mut responses = T::responses(gen)?;
                set_status_code(&mut responses, $status)?;
                Ok(responses)
            }
        }
    };
}

status_responder!(Accepted, 202);
status_responder!(Created, 201);
status_responder!(BadRequest, 400);
// status_responder!(Unauthorized, 401);
// status_responder!(Forbidden, 403);
status_responder!(NotFound, 404);

// impl<'r, T> OpenApiResponderInner<'r> for rocket::response::status::Custom<T>
// where
//     T: OpenApiResponderInner<'r> + Send
// {
//     fn responses(_: &mut OpenApiGenerator) -> Result {
//         let mut responses = Responses::default();
//         set_status_code(&mut responses, xxx)?;
//         Ok(responses)
//     }
// }

macro_rules! response_content_wrapper {
    ($responder: ident, $mime: literal) => {
        impl<T: OpenApiResponderInner> OpenApiResponderInner
            for rocket::response::content::$responder<T>
        {
            fn responses(gen: &mut OpenApiGenerator) -> Result {
                let mut responses = T::responses(gen)?;
                set_content_type(&mut responses, $mime)?;
                Ok(responses)
            }
        }
    };
}

response_content_wrapper!(Css, "text/css");
response_content_wrapper!(Html, "text/html");
response_content_wrapper!(JavaScript, "application/javascript");
response_content_wrapper!(Json, "application/json");
response_content_wrapper!(MsgPack, "application/msgpack");
response_content_wrapper!(Plain, "text/plain");
response_content_wrapper!(Xml, "text/xml");

// impl<'r, T, E> OpenApiResponderInner<'r> for StdResult<T, E>
// where
//     T: OpenApiResponderInner<'r> + Send,
//     E: Debug + Send
// {
//     default fn responses(gen: &mut OpenApiGenerator) -> Result {
//         let mut responses = T::responses(gen)?;
//         ensure_status_code_exists(&mut responses, 500);
//         Ok(responses)
//     }
// }

/*
impl<'r, 'o, T, E> OpenApiResponderInner for StdResult<T, E>
where
    T: OpenApiResponderInner + Send,
    E: Responder<'r, 'o> + Debug + Send + 'r,
{
    default fn responses(_: &mut OpenApiGenerator) -> Result {
        Err(OpenApiError::new("Unable to generate OpenAPI spec for Result<T, E> response, as E implements Responder but not OpenApiResponderInner.".to_owned()))
    }
}
*/

impl<'r, 'o, T, E> OpenApiResponderInner for StdResult<T, E>
where
    T: OpenApiResponderInner,
    E: OpenApiResponderInner + Debug,
{
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let ok_responses = T::responses(gen)?;
        let err_responses = E::responses(gen)?;
        produce_any_responses(ok_responses, err_responses)
    }
}

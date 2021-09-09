use super::OpenApiResponderInner;
use crate::{
    gen::OpenApiGenerator,
    util::{
        add_content_response, add_default_response_code, add_default_response_schema,
        add_schema_response, ensure_status_code_exists, produce_any_responses, set_content_type,
        set_status_code,
    },
};
use okapi::openapi3::Responses;
use rocket::serde::json::{Json, Value};
use schemars::JsonSchema;
use serde::Serialize;

type Result = crate::Result<Responses>;

// Order is same as on:
// https://docs.rs/rocket/0.5.0-rc.1/rocket/response/trait.Responder.html#foreign-impls

impl OpenApiResponderInner for &str {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <String>::responses(gen)
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

impl OpenApiResponderInner for &[u8] {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <Vec<u8>>::responses(gen)
    }
}

impl OpenApiResponderInner for Vec<u8> {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        add_content_response(
            &mut responses,
            200,
            "application/octet-stream",
            okapi::openapi3::MediaType::default(),
        )?;
        Ok(responses)
    }
}

impl OpenApiResponderInner for std::fs::File {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        add_content_response(
            &mut responses,
            200,
            "application/octet-stream",
            okapi::openapi3::MediaType::default(),
        )?;
        Ok(responses)
    }
}

impl OpenApiResponderInner for rocket::tokio::fs::File {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        add_content_response(
            &mut responses,
            200,
            "application/octet-stream",
            okapi::openapi3::MediaType::default(),
        )?;
        Ok(responses)
    }
}

impl OpenApiResponderInner for () {
    fn responses(_: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        ensure_status_code_exists(&mut responses, 200);
        Ok(responses)
    }
}

impl<'r, 'o: 'r, T> OpenApiResponderInner for std::borrow::Cow<'o, T>
where
    T: OpenApiResponderInner + Clone,
{
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = T::responses(gen)?;
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

impl<'r, 'o, T, E> OpenApiResponderInner for std::result::Result<T, E>
where
    T: OpenApiResponderInner,
    E: OpenApiResponderInner,
{
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let ok_responses = T::responses(gen)?;
        let err_responses = E::responses(gen)?;
        produce_any_responses(ok_responses, err_responses)
    }
}

impl<'r, 'o, L, R> OpenApiResponderInner for either::Either<L, R>
where
    L: OpenApiResponderInner,
    R: OpenApiResponderInner,
{
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let left_responses = L::responses(gen)?;
        let right_responses = R::responses(gen)?;
        produce_any_responses(left_responses, right_responses)
    }
}

impl OpenApiResponderInner for std::io::Error {
    fn responses(_gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        ensure_status_code_exists(&mut responses, 500);
        Ok(responses)
    }
}

// The ContentType can be set at runtime, so no way of knowing what the mime-type is up front.
impl<R: OpenApiResponderInner> OpenApiResponderInner for (rocket::http::ContentType, R) {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = R::responses(gen)?;
        set_content_type(&mut responses, rocket::http::ContentType::Any)?;
        Ok(responses)
    }
}

// The Status can be set at runtime, so no way of knowing what the response code is up front.
// This will add "default" response.
impl<R: schemars::JsonSchema> OpenApiResponderInner for (rocket::http::Status, R) {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<R>();
        add_default_response_schema(&mut responses, "*/*", schema);
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

impl OpenApiResponderInner for rocket::fs::NamedFile {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <Vec<u8>>::responses(gen)
    }
}

impl OpenApiResponderInner for rocket::http::Status {
    fn responses(_gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        add_default_response_code(&mut responses);
        Ok(responses)
    }
}

impl OpenApiResponderInner for rocket::response::status::NoContent {
    fn responses(_gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        set_status_code(&mut responses, 204)?;
        Ok(responses)
    }
}

impl OpenApiResponderInner for rocket::response::Redirect {
    fn responses(_gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        // Add all possible status codes.
        set_status_code(&mut responses, 301)?; // Moved Permanently
        set_status_code(&mut responses, 302)?; // Found
        set_status_code(&mut responses, 303)?; // See Other
        set_status_code(&mut responses, 307)?; // Temporary Redirect
        set_status_code(&mut responses, 308)?; // Permanent Redirect
        Ok(responses)
    }
}

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
response_content_wrapper!(Custom, "*/*");
response_content_wrapper!(Html, "text/html");
response_content_wrapper!(JavaScript, "application/javascript");
response_content_wrapper!(Json, "application/json");
response_content_wrapper!(MsgPack, "application/msgpack");
response_content_wrapper!(Plain, "text/plain");
response_content_wrapper!(Xml, "text/xml");

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
status_responder!(BadRequest, 400);
status_responder!(Conflict, 409);
status_responder!(Created, 201);
status_responder!(Custom, 0);
status_responder!(Forbidden, 403);
status_responder!(NotFound, 404);
status_responder!(Unauthorized, 401);

impl<R> OpenApiResponderInner for rocket::response::Flash<R>
where
    R: OpenApiResponderInner,
{
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        R::responses(gen)
    }
}

impl<R> OpenApiResponderInner for rocket::data::Capped<R>
where
    R: OpenApiResponderInner,
{
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        R::responses(gen)
    }
}

/// Debug prints the internal value before forwarding to the 500 error catcher.
impl<E> OpenApiResponderInner for rocket::response::Debug<E>
where
    E: std::fmt::Debug,
{
    fn responses(_gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        set_status_code(&mut responses, 500)?;
        Ok(responses)
    }
}

/// `ByteStream` is a (potentially infinite) responder. The response `Content-Type` is set to `Binary`.
/// The body is unsized, and values are sent as soon as they are yielded by the internal iterator.
impl<S> OpenApiResponderInner for rocket::response::stream::ByteStream<S>
where
    S: Send,
{
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        <Vec<u8>>::responses(gen)
    }
}

/// `ReaderStream` is a (potentially infinite) responder. No `Content-Type` is set.
/// The body is unsized, and values are sent as soon as they are yielded by the internal iterator.
impl<S> OpenApiResponderInner for rocket::response::stream::ReaderStream<S>
where
    S: Send + rocket::futures::Stream,
{
    fn responses(_gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        ensure_status_code_exists(&mut responses, 200);
        Ok(responses)
    }
}

/// `TextStream` is a (potentially infinite) responder. The response `Content-Type` is set to `Text`.
/// The body is unsized, and values are sent as soon as they are yielded by the internal iterator.
impl<S> OpenApiResponderInner for rocket::response::stream::TextStream<S>
where
    S: Send,
{
    fn responses(_gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        add_content_response(
            &mut responses,
            200,
            "text/plain; charset=utf-8",
            okapi::openapi3::MediaType::default(),
        )?;
        Ok(responses)
    }
}

/// Serializes the wrapped value into JSON. Returns a response with `Content-Type` `JSON` and a
/// fixed-size body with the serialized value. If serialization fails,
/// an `Err` of `Status::InternalServerError` is returned.
impl<T: Serialize + JsonSchema + Send> OpenApiResponderInner for Json<T> {
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<T>();
        add_schema_response(&mut responses, 200, "application/json", schema)?;
        // 500 status is not added because an endpoint can handle this, so it might never return
        // this error type.
        Ok(responses)
    }
}

/// Serializes the wrapped value into `MessagePack`. Returns a response with `Content-Type` `MsgPack`
/// and a fixed-size body with the serialization. If serialization fails,
/// an `Err` of `Status::InternalServerError` is returned.
#[cfg(feature = "msgpack")]
impl<T: Serialize + JsonSchema + Send> OpenApiResponderInner
    for rocket::serde::msgpack::MsgPack<T>
{
    fn responses(gen: &mut OpenApiGenerator) -> Result {
        let mut responses = Responses::default();
        let schema = gen.json_schema::<T>();
        add_schema_response(&mut responses, 200, "application/msgpack", schema)?;
        // 500 status is not added because an endpoint can handle this, so it might never return
        // this error type.
        Ok(responses)
    }
}

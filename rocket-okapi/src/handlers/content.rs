use rocket::handler::{Handler, Outcome};
use rocket::http::{ContentType, Method};
use rocket::response::{Content, Responder};
use rocket::{Data, Request, Route};

/// A content handler is a wrapper type around `rocket::response::Content`, which can be turned into
/// a `rocket::Route` that serves the content with correct content-type.
#[derive(Clone)]
pub struct ContentHandler<R: Responder<'static> + Clone + Send + Sync + 'static> {
    content: Content<R>,
}

impl ContentHandler<String> {
    /// Create a `ContentHandler<String>` which serves its content as JSON.
    pub fn json(content: &impl serde::Serialize) -> Self {
        let json =
            serde_json::to_string_pretty(content).expect("Could not serialize content as JSON.");
        ContentHandler {
            content: Content(ContentType::JSON, json),
        }
    }
}

impl ContentHandler<&'static [u8]> {
    /// Create a `ContentHandler<&[u8]>`, which serves its content with the specified
    /// `content_type`.
    pub fn bytes(content_type: ContentType, content: &'static [u8]) -> Self {
        ContentHandler {
            content: Content(content_type, content),
        }
    }
}

impl<R: Responder<'static> + Clone + Send + Sync + 'static> ContentHandler<R> {
    /// Create a `rocket::Route` from the current `ContentHandler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }
}

impl<R: Responder<'static> + Clone + Send + Sync + 'static> Handler for ContentHandler<R> {
    fn handle<'r>(&self, req: &'r Request, data: Data) -> Outcome<'r> {
        // match e.g. "/index.html" but not "/index.html/"
        if req.uri().path().ends_with('/') {
            Outcome::Forward(data)
        } else {
            Outcome::from(req, self.content.clone())
        }
    }
}

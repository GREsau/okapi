use rocket::http::{ContentType, Method};
use rocket::response::{content::Custom, Responder};
use rocket::route::{Handler, Outcome};
use rocket::{Data, Request, Route};

/// A content handler is a wrapper type around `rocket::response::Content`, which can be turned into
/// a `rocket::Route` that serves the content with correct content-type.
#[derive(Clone)]
pub struct ContentHandler<R: AsRef<[u8]> + Clone + Send + Sync> {
    content: Custom<R>,
}

impl ContentHandler<String> {
    /// Create a `ContentHandler<String>` which serves its content as JSON.
    pub fn json(content: &impl serde::Serialize) -> Self {
        let json =
            serde_json::to_string_pretty(content).expect("Could not serialize content as JSON.");
        ContentHandler {
            content: Custom(ContentType::JSON, json),
        }
    }
}

impl ContentHandler<&'static [u8]> {
    /// Create a `ContentHandler<&[u8]>`, which serves its content with the specified
    /// `content_type`.
    #[must_use]
    pub fn bytes(content_type: ContentType, content: &'static [u8]) -> Self {
        ContentHandler {
            content: Custom(content_type, content),
        }
    }
}

impl<R: AsRef<[u8]> + Clone + Send + Sync + 'static> ContentHandler<R> {
    /// Create a `rocket::Route` from the current `ContentHandler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path.as_ref(), self)
    }
}

#[rocket::async_trait]
impl<R> Handler for ContentHandler<R>
where
    R: AsRef<[u8]> + Clone + Send + Sync + 'static,
{
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        // match e.g. "/index.html" but not "/index.html/"
        if req.uri().path().ends_with('/') {
            Outcome::forward(data)
        } else {
            let content: Custom<Vec<u8>> =
                Custom(self.content.0.clone(), self.content.1.as_ref().into());
            match content.respond_to(req) {
                Ok(response) => Outcome::Success(response),
                Err(status) => Outcome::Failure(status),
            }
        }
    }
}

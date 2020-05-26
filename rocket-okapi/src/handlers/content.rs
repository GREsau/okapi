use rocket::handler::{Handler, HandlerFuture, Outcome};
use rocket::http::{ContentType, Method};
use rocket::response::{Content, Responder};
use rocket::{Data, Request, Route};

/// A content handler is a wrapper type around `rocket::response::Content`, which can be turned into
/// a `rocket::Route` that serves the content with correct content-type.
#[derive(Clone)]
pub struct ContentHandler<R: AsRef<[u8]> + Clone + Send + Sync> {
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

impl<R: AsRef<[u8]> + Clone + Send + Sync + 'static> ContentHandler<R> {
    /// Create a `rocket::Route` from the current `ContentHandler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }
}

impl<R> Handler for ContentHandler<R>
where
    R: AsRef<[u8]> + Clone + Send + Sync + 'static,
{
    fn handle<'r>(&self, req: &'r Request<'_>, data: Data) -> HandlerFuture<'r> {
        // match e.g. "/index.html" but not "/index.html/"
        if req.uri().path().ends_with('/') {
            Box::pin(async { Outcome::forward(data) })
        } else {
            let content: Content<Vec<u8>> = Content(self.content.0.clone(), self.content.1.as_ref().into());
            Box::pin(async move { 
                match content.respond_to(req).await {
                    Ok(response) => Outcome::Success(response),
                    Err(status) => Outcome::Failure(status),
                }
            })
        }
    }
}

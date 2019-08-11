use rocket::handler::{Handler, Outcome};
use rocket::http::{ContentType, Method};
use rocket::response::Content;
use rocket::{Data, Request, Route};

#[derive(Clone)]
pub struct ContentHandler(Content<String>);

impl ContentHandler {
    pub fn json(content: &impl serde::Serialize) -> Self {
        let json =
            serde_json::to_string_pretty(content).expect("Could not serialize content as JSON.");
        ContentHandler(Content(ContentType::JSON, json))
    }

    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }
}

impl Handler for ContentHandler {
    fn handle<'r>(&self, req: &'r Request, _: Data) -> Outcome<'r> {
        Outcome::from(req, self.0.clone())
    }
}

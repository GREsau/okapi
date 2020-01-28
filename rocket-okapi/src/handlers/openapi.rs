use okapi::openapi3::{OpenApi, Server};
use rocket::handler::{Handler, Outcome};
use rocket::http::{ContentType, Method};
use rocket::response::Content;
use rocket::{Data, Request, Route};

/// A handler type that is used to serve the `openapi.json` files.
#[derive(Clone)]
pub struct OpenApiHandler {
    spec: OpenApi,
}

impl OpenApiHandler {
    /// Create a new handler from an API spec.
    pub fn new(spec: OpenApi) -> Self {
        OpenApiHandler { spec }
    }

    /// Create a new route from this `OpenApiHandler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }
}

impl Handler for OpenApiHandler {
    fn handle<'r>(&self, req: &'r Request, _: Data) -> Outcome<'r> {
        let mut spec = self.spec.clone();
        let base_path = req
            .route()
            .expect("Routing should already have occurred.")
            .base();

        if spec.servers.is_empty() && base_path != "/" {
            spec.servers.push(Server {
                url: base_path.to_owned(),
                ..Default::default()
            })
        }

        let json =
            serde_json::to_string_pretty(&spec).expect("Could not serialize content as JSON.");
        Outcome::from(req, Content(ContentType::JSON, json))
    }
}

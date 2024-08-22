use okapi::openapi3::{OpenApi, Server};
use rocket::http::Method;
use rocket::response::content::RawJson;
use rocket::route::{Handler, Outcome};
use rocket::{Data, Request, Route};

/// A handler type that is used to serve the `openapi.json` files.
#[derive(Clone)]
pub struct OpenApiHandler {
    spec: OpenApi,
}

impl OpenApiHandler {
    /// Create a new handler from an API spec.
    #[must_use]
    pub fn new(spec: OpenApi) -> Self {
        OpenApiHandler { spec }
    }

    /// Create a new route from this `OpenApiHandler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path.as_ref(), self)
    }
}

#[rocket::async_trait]
impl Handler for OpenApiHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {
        let mut spec = self.spec.clone();
        let base_path = req
            .route()
            .expect("Routing should already have occurred.")
            .uri
            .base();

        if spec.servers.is_empty() && base_path != "/" {
            spec.servers.push(Server {
                url: base_path.to_string(),
                ..okapi::openapi3::Server::default()
            });
        }

        let json =
            serde_json::to_string_pretty(&spec).expect("Could not serialize content as JSON.");
        Outcome::from(req, RawJson(json))
    }
}

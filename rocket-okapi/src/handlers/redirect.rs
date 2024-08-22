use rocket::http::Method;
use rocket::response::Redirect;
use rocket::route::{Handler, Outcome};
use rocket::{Data, Request, Route};

/// A handler that instead of serving content always redirects to some specified destination URL.
#[derive(Clone)]
pub struct RedirectHandler {
    dest: &'static str,
}

impl RedirectHandler {
    /// Create a new `RedirectHandler` that redirects to the specified URL.
    #[must_use]
    pub fn to(dest: &'static str) -> Self {
        Self {
            dest: dest.trim_start_matches('/'),
        }
    }

    /// Create a new `Route` from this `Handler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path.as_ref(), self)
    }
}

#[rocket::async_trait]
impl Handler for RedirectHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {
        let path = req
            .route()
            .unwrap()
            .uri
            .base()
            .to_string()
            .trim_end_matches('/')
            .to_string();
        Outcome::from(req, Redirect::to(format!("{}/{}", path, self.dest)))
    }
}

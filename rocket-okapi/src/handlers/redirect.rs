use rocket::handler::{Handler, Outcome};
use rocket::http::Method;
use rocket::response::Redirect;
use rocket::{Data, Request, Route};

/// A handler that instead of serving content always redirects to some specified destination URL.
#[derive(Clone)]
pub struct RedirectHandler {
    dest: &'static str,
}

impl RedirectHandler {
    /// Create a new `RedirectHandler` that redirects to the specified URL.
    pub fn to(dest: &'static str) -> Self {
        Self {
            dest: dest.trim_start_matches('/'),
        }
    }

    /// Create a new `Route` from this `Handler`.
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }
}

#[rocket::async_trait]
impl Handler for RedirectHandler {
    async fn handle<'r, 's: 'r>(&'s self, req: &'r Request<'_>, _: Data) -> Outcome<'r> {
        let path = req.route().unwrap().base().trim_end_matches('/');
        Outcome::from(req, Redirect::to(format!("{}/{}", path, self.dest)))
    }
}

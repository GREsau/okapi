use rocket::handler::{Handler, Outcome};
use rocket::http::{ContentType, Method};
use rocket::response::{Content, Responder};
use rocket::{Data, Request, Route};

#[derive(Clone)]
pub struct ContentHandler<R: Responder<'static> + Clone + Send + Sync + 'static>(Content<R>);

macro_rules! static_file {
    ($name: literal, $type: ident) => {
        ContentHandler(Content::<&'static [u8]>(
            ContentType::$type,
            include_bytes!(concat!("../swagger-ui/", $name)),
        ))
        .into_route(concat!("/swagger/", $name))
    };
}

pub fn swagger_ui_routes() -> Vec<Route> {
    vec![
        static_file!("favicon-16x16.png", PNG),
        static_file!("favicon-32x32.png", PNG),
        static_file!("index.html", HTML),
        static_file!("oauth2-redirect.html", HTML),
        static_file!("swagger-ui.js", JavaScript),
        static_file!("swagger-ui-standalone-preset.js", JavaScript),
        static_file!("swagger-ui-bundle.js", JavaScript),
        static_file!("swagger-ui.css", CSS),
    ]
}

impl ContentHandler<String> {
    pub fn json(content: &impl serde::Serialize) -> Self {
        let json =
            serde_json::to_string_pretty(content).expect("Could not serialize content as JSON.");
        ContentHandler(Content(ContentType::JSON, json))
    }
}

impl<R: Responder<'static> + Clone + Send + Sync + 'static> ContentHandler<R> {
    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }
}

impl<R: Responder<'static> + Clone + Send + Sync + 'static> Handler for ContentHandler<R> {
    fn handle<'r>(&self, req: &'r Request, _: Data) -> Outcome<'r> {
        Outcome::from(req, self.0.clone())
    }
}

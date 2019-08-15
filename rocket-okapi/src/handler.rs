use rocket::handler::{Handler, Outcome};
use rocket::http::{ContentType, Method};
use rocket::response::Content;
use rocket::{Data, Request, Route};

// TODO allow &[u8]
#[derive(Clone)]
pub struct ContentHandler(Content<String>);

macro_rules! static_file {
    ($name: literal, $type: ident) => {
        ContentHandler(Content(
            ContentType::$type,
            include_str!(concat!("../swagger-ui/", $name)).to_owned(),
        ))
        .into_route(concat!("/swagger/", $name))
    };
}

impl ContentHandler {
    pub fn json(content: &impl serde::Serialize) -> Self {
        let json =
            serde_json::to_string_pretty(content).expect("Could not serialize content as JSON.");
        ContentHandler(Content(ContentType::JSON, json))
    }

    pub fn into_route(self, path: impl AsRef<str>) -> Route {
        Route::new(Method::Get, path, self)
    }

    pub fn swagger_ui_routes() -> Vec<Route> {
        // TODO binary (PNG) files
        vec![
            static_file!("index.html", HTML),
            static_file!("oauth2-redirect.html", HTML),
            static_file!("swagger-ui.js", JavaScript),
            static_file!("swagger-ui-standalone-preset.js", JavaScript),
            static_file!("swagger-ui-bundle.js", JavaScript),
            static_file!("swagger-ui.css", CSS),
        ]
    }
}

impl Handler for ContentHandler {
    fn handle<'r>(&self, req: &'r Request, _: Data) -> Outcome<'r> {
        Outcome::from(req, self.0.clone())
    }
}

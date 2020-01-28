use crate::handlers::{ContentHandler, RedirectHandler};
use serde::{Deserialize, Serialize};

use rocket::http::ContentType;
use rocket::Route;

macro_rules! static_file {
    ($name: literal, $type: ident) => {
        ContentHandler::bytes(
            ContentType::$type,
            include_bytes!(concat!("../swagger-ui/", $name)),
        )
        .into_route(concat!("/", $name))
    };
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerUIConfig {
    /// The url to the default `openapi.json` file that is showed when the web ui is first opened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A list of named urls that contain all the `openapi.json` files that you want to display in
    /// your web ui.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlObject>>,
}

/// Contains a named url.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlObject {
    /// The name of the url.
    pub name: String,
    /// The url itself.
    pub url: String,
}

/// Transform the provided `SwaggerUIConfig` into a list of `Route`s that serve the swagger web ui.
pub fn make_swagger_ui(config: &SwaggerUIConfig) -> impl Into<Vec<Route>> {
    let config_handler = ContentHandler::json(config);
    vec![
        config_handler.into_route("/swagger-ui-config.json"),
        RedirectHandler::to("index.html").into_route("/"),
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

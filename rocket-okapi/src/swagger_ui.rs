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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerUIConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlObject>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlObject {
    pub name: String,
    pub url: String,
}

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

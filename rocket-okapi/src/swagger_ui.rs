use crate::handlers::{ContentHandler, RedirectHandler};
use crate::settings::UrlObject;
use rocket::http::ContentType;
use rocket::Route;
use serde::{Deserialize, Serialize};

macro_rules! static_file {
    ($name: literal, $type: ident) => {
        ContentHandler::bytes(
            ContentType::$type,
            include_bytes!(concat!("../swagger-ui/", $name)),
        )
        .into_route(concat!("/", $name))
    };
}

// To update this structures use;
// https://github.com/swagger-api/swagger-ui/blob/master/docs/usage/configuration.md

/// Used to control the way models are displayed by default.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DefaultModelRendering {
    /// Expand the `example` section.
    Example,
    /// Expand the `model` section.
    Model,
}

/// Used to control the default expansion setting for the operations and tags.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DocExpansion {
    /// Expands only the tags.
    List,
    /// Expands the tags and operations
    Full,
    /// Expands nothing
    None,
}

/// Used to enable, disable and preconfigure filtering
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Filter {
    /// Use this variant to enable or disable filtering.
    Bool(bool),
    /// Use this variant to enable filtering, and preconfigure a filter.
    Str(String),
}

fn is_zero(num: &u32) -> bool {
    *num == 0
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerUIConfig {
    /// The url to a single `openapi.json` file that is showed when the web ui is first opened.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
    /// A list of named urls that contain all the `openapi.json` files that you want to display in
    /// your web ui. If this field is populated, the `url` field is not used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<UrlObject>,
    // display options:
    /// If set to true, enables deep linking for tags and operations. See the
    /// [Deep Linking documentation](https://github.com/swagger-api/swagger-ui/blob/master/docs/usage/deep-linking.md)
    /// for more information.
    /// Default: `false`.
    pub deep_linking: bool,
    /// Controls the display of operationId in operations list.
    /// Default: `false`.
    pub display_operation_id: bool,
    /// The default expansion depth for models (set to -1 completely hide the models).
    /// Default: `1`.
    pub default_models_expand_depth: i32,
    /// The default expansion depth for the model on the model-example section.
    /// Default: `1`.
    pub default_model_expand_depth: i32,
    /// Controls how the model is shown when the API is first rendered. (The user can always switch
    /// the rendering for a given model by clicking the 'Model' and 'Example Value' links.)
    /// Default: `DefaultModelRendering::Example`.
    pub default_model_rendering: DefaultModelRendering,
    /// Controls the display of the request duration (in milliseconds) for "Try it out" requests.
    /// Default: `false`.
    pub display_request_duration: bool,
    /// Controls the default expansion setting for the operations and tags.
    /// Default: `DocExpansion::List`.
    pub doc_expansion: DocExpansion,
    /// If set, enables filtering. The top bar will show an edit box that you can use to filter the
    /// tagged operations that are shown. Filtering is case sensitive matching the filter expression
    /// anywhere inside the tag.
    /// Default: `Filter(false)`.
    pub filter: Filter,
    /// If set, limits the number of tagged operations displayed to at most this many. The default
    /// is to show all operations.
    /// Default: `None` (displays all tagged operations).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub max_displayed_tags: u32,
    /// Controls the display of vendor extension (`x-`) fields and values for Operations,
    /// Parameters, and Schema.
    /// Default: `false`.
    pub show_extensions: bool,
    /// Controls the display of extensions (`pattern`, `maxLength`, `minLength`, `maximum`,
    /// `minimum`) fields and values for Parameters.
    /// Default: `false`.
    pub show_common_extensions: bool,
}

impl Default for SwaggerUIConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            urls: vec![],
            deep_linking: false,
            display_operation_id: false,
            default_model_expand_depth: 1,
            default_model_rendering: DefaultModelRendering::Example,
            default_models_expand_depth: 1,
            display_request_duration: false,
            doc_expansion: DocExpansion::List,
            filter: Filter::Bool(false),
            max_displayed_tags: 0,
            show_extensions: false,
            show_common_extensions: false,
        }
    }
}

/// Transform the provided `SwaggerUIConfig` into a list of `Route`s that serve the swagger web ui.
#[must_use]
pub fn make_swagger_ui(config: &SwaggerUIConfig) -> impl Into<Vec<Route>> {
    let config_handler = ContentHandler::json(config);
    vec![
        RedirectHandler::to("index.html").into_route("/"),
        // Add custom config file
        config_handler.into_route("/swagger-ui-config.json"),
        // Add other static files
        static_file!("index.html", HTML),
        static_file!("index.css", CSS),
        static_file!("oauth2-redirect.html", HTML),
        static_file!("swagger-initializer.js", JavaScript),
        static_file!("swagger-ui-standalone-preset.js", JavaScript),
        static_file!("swagger-ui-bundle.js", JavaScript),
        static_file!("swagger-ui.css", CSS),
    ]
}

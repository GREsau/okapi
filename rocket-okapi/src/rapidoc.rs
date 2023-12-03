//! ### Example
//! ```rust,no_run
//! use rocket_okapi::settings::UrlObject;
//! use rocket_okapi::rapidoc::{make_rapidoc, RapiDocConfig, GeneralConfig};
//!
//! fn get_rapi_docs() -> RapiDocConfig {
//!     RapiDocConfig {
//!         general: GeneralConfig {
//!             spec_urls: get_urls(), // this is the only required field
//!             ..Default::default()
//!         },
//!         ..Default::default()
//!     }
//! }
//!
//! fn get_urls() -> Vec<UrlObject> {
//!     vec![
//!         UrlObject::new("Resource", "/my_resource/openapi.json"),
//!     ]
//! }
//!
//! #[rocket::main]
//! async fn main() {
//!     rocket::build()
//!         .mount("/rapi-doc", make_rapidoc(&get_rapi_docs()))
//!         .launch()
//!         .await
//!         .unwrap();
//! }
//! ```

use crate::handlers::{ContentHandler, RedirectHandler};
use crate::settings::UrlObject;
use rocket::http::ContentType;
use rocket::Route;
use std::collections::HashMap;

macro_rules! static_file {
    ($name: literal, $type: ident) => {
        ContentHandler::bytes(
            ContentType::$type,
            include_bytes!(concat!("../rapidoc/", $name)),
        )
        .into_route(concat!("/", $name))
    };
}

/// Macro to crate a `HashMap` with a number of key-value pairs in it.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use rocket_okapi::hash_map;
///
/// let my_hash_map = hash_map!{
///     "token_name".to_owned() => "CREATURE",
///     "cat".to_owned() => "",
/// };
///
/// let mut control = HashMap::new();
/// control.insert("token_name".to_owned(),"CREATURE");
/// control.insert("cat".to_owned(),"");
///
/// assert_eq!(my_hash_map, control);
/// ```
#[macro_export]
macro_rules! hash_map {
    ($($key:expr => $val:expr),* $(,)*) => ({
        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    });
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone, Default)]
pub struct RapiDocConfig {
    /// Webpage title. An optional title for the webpage.
    /// If set to `None` we will create a default title.
    pub title: Option<String>,
    /// General settings. The `spec_urls` property _must_ be specified by the user.
    pub general: GeneralConfig,
    /// Settings related to the ui and theming.
    pub ui: UiConfig,
    /// Settings related to the nav bar.
    pub nav: NavConfig,
    /// Settings relatd to the layout of the displayed docs.
    pub layout: LayoutConfig,
    /// Settings used to control what features should or should not be displayed.
    pub hide_show: HideShowConfig,
    /// Settings around the Schema view.
    pub schema: SchemaConfig,
    /// Settings used to configure access to the api.
    pub api: ApiConfig,
    /// Settings to configure the Rapi Doc "slots".
    pub slots: SlotsConfig,
    /// Provide a custom HTML file content.
    /// The templated values will still be replaced. So other settings can still be used.
    /// Use [../rapidoc/index.html](../rapidoc/index.html) as an example.
    pub custom_html: Option<String>,
    /// A list of custom tags that can be used in combination with `custom_html`.
    /// This allows for additional custom template tags that will be replaced in the html.
    /// The key should be the name of the tag without the brackets, `{{key}}`.
    /// The value will be the text the value will be replaced with.
    ///
    /// The custom tags are replaced before all other tags, this allows for more flexibility
    /// but also means that you can break things. If you want to be sure to not overlap with
    /// existing tags, prefix your custom tags with `_`, `c_` or `C_`.
    /// We will never use these prefixes in the provided tags.
    pub custom_template_tags: HashMap<String, String>,
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone)]
pub struct GeneralConfig {
    /// Urls of the OpenAPI spec to view.
    ///
    /// This field _must_ be manually filled with at least one element.
    /// More then one element is currently not supported yet, but can be used with custom HTML.
    // This is different as the default spec on https://rapidocweb.com/api.html
    // That is by design. This does not have to be changed.
    pub spec_urls: Vec<UrlObject>,
    /// Setting true will update the url on browser's location whenever a new section
    /// is visited either by scrolling or clicking.
    pub update_route: bool,
    /// routes for each operation/api is generated based on the api path.
    /// however you may add a custom prefix to these routes to support third party routing needs.
    ///
    /// The default is "#".
    pub route_prefix: String,
    /// To list tags in alphabetic order, otherwise tags will be ordered based on how it is
    /// specified under the tags section in the spec.
    ///
    /// The default is `false`.
    pub sort_tags: bool,
    /// Sort endpoints within each tags by path or method.
    ///
    /// The default is [`SortEndpointsBy::Path`].
    pub sort_endpoints_by: SortEndpointsBy,
    /// Heading Text on top-left corner.
    pub heading_text: String,
    /// Initial location on the document(identified by method and path) where you want to go after
    /// the spec is loaded. `goto_path` should be in the form of {method}-{path}. For instance you
    /// want to scrollTo "GET /user/login" you should provide the location as `get-/user/login`.
    pub goto_path: String,
    /// Request fields will be filled with example value (if provided in spec).
    ///
    /// The default is `true`.
    pub fill_request_fields_with_example: bool,
    /// Authentication will be persisted to localStorage.
    ///
    /// The default is `false`.
    pub persist_auth: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            spec_urls: vec![],
            update_route: true,
            route_prefix: "#".to_owned(),
            sort_tags: false,
            sort_endpoints_by: SortEndpointsBy::Path,
            heading_text: "".to_owned(),
            goto_path: "".to_owned(),
            fill_request_fields_with_example: true,
            persist_auth: false,
        }
    }
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone)]
pub struct UiConfig {
    /// Is the base theme, which is used for calculating colors for various UI components. 'theme',
    /// 'bg-color' and 'text-color' are the base attributes for generating a custom theme.
    ///
    /// The default is [`Theme::Light`].
    pub theme: Theme,
    /// Hex color code for main background.
    pub bg_color: String,
    /// Hex color code for text.
    pub text_color: String,
    /// Hex color code for the header's background.
    pub header_color: String,
    /// Hex color code on various controls such as buttons, tabs.
    pub primary_color: String,
    /// RapiDoc will attempt to load fonts from CDN, if this is not intended, then set this to false.
    ///
    /// The default is `true`.
    pub load_fonts: bool,
    /// Font Name(s) to be used for regular text.
    pub regular_font: String,
    /// Font Name(s) to be used for mono-spaced text.
    pub mono_font: String,
    /// Sets the relative font sizes for the entire document.
    ///
    /// The default is [`FontSize::Default`].
    pub font_size: FontSize,
    /// Create a CSS file that will be `<link>`ed in the HTML file.
    ///
    /// Used together with [`UiConfig::css_classes`]
    pub css_file: Option<String>,
    /// Provide names of all the CSS class names that you would like to apply to RapiDoc Element.
    /// Note: Space (` `) character is not allowed, this is used as a separator.
    ///
    /// Used together with [`UiConfig::css_file`]
    pub css_classes: Vec<String>,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            bg_color: "".to_owned(),
            text_color: "".to_owned(),
            header_color: "".to_owned(),
            primary_color: "".to_owned(),
            load_fonts: true,
            regular_font: "".to_owned(),
            mono_font: "".to_owned(),
            font_size: FontSize::Default,
            css_file: None,
            css_classes: Vec::new(),
        }
    }
}

/// A struct containing information about where and how the `openapi.json` files are served.
///
/// Only applicable if [`LayoutConfig::render_style`] is set to [`RenderStyle::Read`] or
/// [`RenderStyle::Focused`].
#[derive(Debug, Clone)]
pub struct NavConfig {
    /// shows API Method names in the navigation bar
    /// (if you customized nav-background make sure there is a proper contrast).
    ///
    /// Example: <https://rapidocweb.com/examples/nav-item-with-method.html>
    pub show_method_in_nav_bar: ShowMethodInNavBar,
    /// Set true to show API paths in the navigation bar instead of summary/description.
    ///
    /// The default is `false`.
    /// Example: <https://rapidocweb.com/examples/nav-item-as-path.html>
    pub use_path_in_nav_bar: bool,
    /// Navigation bar's background color.
    ///
    /// Example: <https://rapidocweb.com/examples/nav-bg-color.html>
    pub nav_bg_color: String,
    /// Navigation bar's Text color.
    pub nav_text_color: String,
    /// Background color of the navigation item on mouse-over.
    pub nav_hover_bg_color: String,
    /// Text color of the navigation item on mouse-over.
    pub nav_hover_text_color: String,
    /// Accent color used in navigation Bar (such as background of active navigation item).
    pub nav_accent_color: String,
    /// Text color used in navigation bar selected items.
    pub nav_accent_text_color: String,
    /// Navigation active item indicator styles.
    ///
    /// The default is [`NavActiveItemMarker::LeftBar`].
    pub nav_active_item_marker: NavActiveItemMarker,
    /// Controls navigation item spacing.
    ///
    /// The default is [`NavItemSpacing::Default`].
    /// Example: <https://rapidocweb.com/examples/navbar-spacing.html>
    pub nav_item_spacing: NavItemSpacing,
    /// Applies only to focused render-style. It determines the behavior of clicking on a
    /// Tag in navigation bar. It can either expand-collapse the tag or take you to the tag's
    /// description page.
    ///
    /// The default is [`NavTagClick::ExpandCollapse`].
    pub on_nav_tag_click: NavTagClick,
}

impl Default for NavConfig {
    fn default() -> Self {
        Self {
            show_method_in_nav_bar: ShowMethodInNavBar::None,
            use_path_in_nav_bar: false,
            nav_bg_color: "".to_owned(),
            nav_text_color: "".to_owned(),
            nav_hover_bg_color: "".to_owned(),
            nav_hover_text_color: "".to_owned(),
            nav_accent_color: "".to_owned(),
            nav_accent_text_color: "".to_owned(),
            nav_active_item_marker: NavActiveItemMarker::LeftBar,
            nav_item_spacing: NavItemSpacing::Default,
            on_nav_tag_click: NavTagClick::ExpandCollapse,
        }
    }
}

/// Used to customize the layout of the displayed docs.
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    /// Layout helps in placement of request/response sections. In column layout, request & response
    /// sections are placed one below the other, In row layout they are placed side by side. This
    /// attribute is applicable only when the device width is more than 768px and the render-style
    /// is 'view'.
    ///
    /// The default is `Layout::Row`.
    pub layout: Layout,
    /// Determines display of api-docs. Currently there are two modes supported.
    ///
    /// - `view` friendly for quick exploring (expand/collapse the section of your interest)
    /// - `read` suitable for reading (like a continuous web-page)
    /// - `focused` similar to read but focuses on a single endpoint at a time (good for large specs)
    ///
    /// `read` is more suitable for reading, `view` is more friendly for quick exploring.
    ///
    /// The default is [`RenderStyle::Read`].
    pub render_style: RenderStyle,
    /// Use this value to control the height of response textarea.
    ///
    /// Allowed: valid css height value such as `400px`, `50%`, `60vh`, etc.
    /// The default is `"300px"`.
    pub response_area_height: String,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            layout: Layout::Row,
            render_style: RenderStyle::Read,
            response_area_height: "300px".to_owned(),
        }
    }
}

/// Used to configure what features to hide or show.
#[derive(Debug, Clone)]
pub struct HideShowConfig {
    /// show/hide the documents info section
    /// Info section contains information about the spec, such as the title and description of the
    /// spec, the version, terms of services etc. In certain situation you may not need to show this
    /// section. For instance you are embedding this element inside a another help document. Chances
    /// are, the help doc may already have this info, in that case you may want to hide this
    /// section.
    ///
    /// The default is `true`.
    pub show_info: bool,
    /// Include headers from info -> description section to the Navigation bar (applies to read mode
    /// only)
    /// Will get the headers from the markdown in info - description (h1 and h2) into the menu on
    /// the left (in read mode) along with links to them. This option allows users to add navigation
    /// bar items using Markdown.
    ///
    /// The default is `false`.
    pub info_description_headings_in_navbar: bool,
    /// show/hide the components section both in document and menu
    /// Will show the components section along with schemas, responses, examples, requestBodies,
    /// headers, securitySchemes, links and callbacks Also will be shown in the menu on the left (in
    /// read mode)
    ///
    /// The default is `false`.
    pub show_components: bool,
    /// show/hide the header.
    /// If you do not want your user to open any other api spec, other than the current one, then
    /// set this attribute to `false`.
    ///
    /// The default is `true`.
    pub show_header: bool,
    /// Authentication feature, allows the user to select one of the authentication mechanism thats
    /// available in the spec. It can be http-basic, http-bearer or api-key. If you do not want your
    /// users to go through the authentication process, instead want them to use a pre-generated
    /// api-key then you may hide authentication section by setting this attribute to false and
    /// provide the api-key details using various api-key-???? attributes.
    ///
    /// The default is `true`.
    pub allow_authentication: bool,
    /// If set to `false`, user will not be able to load any spec url from the UI.
    ///
    /// The default is `true`.
    pub allow_spec_url_load: bool,
    /// If set to 'false', user will not be able to load any spec file from the local drive. This
    /// attribute is applicable only when the device width is more than 768px, else this feature is
    /// not available.
    ///
    /// The default is `true`.
    pub allow_spec_file_load: bool,
    /// If set to 'true', it provide buttons in the overview section to download the spec
    /// or open it in a new tab.
    pub allow_spec_file_download: bool,
    /// If set to `false`, user will not be able to search APIs.
    ///
    /// The default is `true`.
    pub allow_search: bool,
    /// Provides advanced search functionality, to search through API-paths, API-description,
    /// API-parameters and API-Responses.
    ///
    /// Example: <https://rapidocweb.com/examples/search-by-params.html>
    pub allow_advanced_search: bool,
    /// 'TRY' feature allows you to make REST calls to the API server. To disable this feature set
    /// it to false
    /// Setting it to false will also hide API-Servers if specified in the spec.
    ///
    /// The default is `true`.
    /// Example: <https://rapidocweb.com/examples/try.html>
    pub allow_try: bool,
    /// If set to 'true', the cURL snippet is displayed between the request and the response
    /// without clicking on TRY.
    ///
    /// Example: <https://rapidocweb.com/examples/show-curl-before-try.html>
    pub show_curl_before_try: bool,
    /// If set to 'false', user will not be able to see or select API server (Server List will be
    /// hidden, however users will be able to see the server url near the 'TRY' button, to know in
    /// advance where the TRY will send the request). The URL specified in the server-url attribute
    /// will be used if set, else the first server in the API specification file will be used.
    ///
    /// The default is `true`.
    pub allow_server_selection: bool,
    /// Allow or hide the ability to expand/collapse field descriptions in the schema.
    ///
    /// The default is `true`.
    pub allow_schema_description_expand_toggle: bool,
}

impl Default for HideShowConfig {
    fn default() -> Self {
        Self {
            show_info: true,
            info_description_headings_in_navbar: false,
            show_components: false,
            show_header: true,
            allow_authentication: true,
            allow_spec_url_load: true,
            allow_spec_file_load: true,
            allow_spec_file_download: false,
            allow_search: true,
            allow_advanced_search: true,
            allow_try: true,
            show_curl_before_try: false,
            allow_server_selection: true,
            allow_schema_description_expand_toggle: true,
        }
    }
}

/// Settings around the Schema view.
#[derive(Debug, Clone)]
pub struct SchemaConfig {
    /// Two different ways to display object-schemas in the responses and request bodies.
    ///
    /// The default is [`SchemaStyle::Tree`].
    pub schema_style: SchemaStyle,
    /// Schemas are expanded by default, use this attribute to control how many levels in the schema
    /// should be expanded.
    ///
    /// The default is `999`.
    pub schema_expand_level: u16,
    /// Constraint and descriptions information of fields in the schema are collapsed to show only
    /// the first line. Set it to true if you want them to fully expanded.
    ///
    /// The default is `false`.
    pub schema_description_expanded: bool,
    /// Read-only fields in request schemas is always hidden but are shown in response.
    /// If you do not want to hide read-only fields or hide them based on action you can configure
    /// this setting to 'never' or any combination of post | put | patch to indicate where to hide
    /// Schemas in response section is not affected by this setting.
    ///
    /// The default is [`SchemaHideReadOnly::Always`].
    /// TODO: This seems to have changed, further changed might be needed.
    pub schema_hide_read_only: SchemaHideReadOnly,
    /// Write-only fields in response schemas is always hidden but are shown in request.
    /// If you do not want to hide write-only fields then set to 'never'
    /// Schemas in request section is not affected by this setting.
    ///
    /// The default is [`SchemaHideWriteOnly::Always`].
    /// TODO: This seems to have changed, further changed might be needed.
    pub schema_hide_write_only: SchemaHideWriteOnly,
    /// The schemas are displayed in two tabs - Model and Example. This option allows you to pick
    /// the default tab that you would like to be active.
    ///
    /// The default is [`DefaultSchemaTab::Model`].
    pub default_schema_tab: DefaultSchemaTab,
}

impl Default for SchemaConfig {
    fn default() -> Self {
        Self {
            schema_style: SchemaStyle::Tree,
            schema_expand_level: 999,
            schema_description_expanded: false,
            schema_hide_read_only: SchemaHideReadOnly::Always,
            schema_hide_write_only: SchemaHideWriteOnly::Always,
            default_schema_tab: DefaultSchemaTab::Model,
        }
    }
}

/// Used to configure api access.
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// OpenAPI spec has a provision for providing the server url. The UI will list all the server
    /// URLs provided in the spec. The user can then select one URL to which he or she intends to
    /// send API calls while trying out the apis. However, if you want to provide an API server of
    /// your own which is not listed in the spec, you can use this property to provide one. It is
    /// helpful in the cases where the same spec is shared between multiple environment say Dev and
    /// Test and each have their own API server.
    pub server_url: String,
    /// If you have multiple api-server listed in the spec, use this attribute to select the default
    /// API server, where all the API calls will goto. This can be changed later from the UI.
    pub default_api_server: String,
    /// Name of the API key that will be send while trying out the APIs.
    ///
    /// The default is "".
    pub api_key_name: String,
    /// Determines how you want to send the api-key.
    ///
    /// The default is `None`.
    pub api_key_location: Option<ApiKeyLocation>,
    /// Value of the API key that will be send while trying out the APIs. This can also be
    /// provided/overwritten from UI.
    pub api_key_value: String,
    /// Enables passing credentials/cookies in cross domain calls,
    /// as defined in the Fetch standard, in CORS requests that are sent by the browser.
    ///
    /// The default is `None`.
    pub fetch_credentials: Option<FetchCredentials>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            server_url: "".to_owned(),
            default_api_server: "".to_owned(),
            api_key_name: "".to_owned(),
            api_key_location: None,
            api_key_value: "".to_owned(),
            fetch_credentials: None,
        }
    }
}

/// Config used to configure the slots.
/// Each slot usually corresponds with one section.
/// Each field can include any HTML tags.
///
/// For an example of all the slots locations see: <https://mrin9.github.io/RapiDoc/examples/slots.html>
#[derive(Debug, Clone, Default)]
pub struct SlotsConfig {
    /// Any content here will be shown immediately under the header and above the info section.
    pub default: Vec<String>,
    /// An image used as the page logo.
    /// This can contain:
    /// - An URL to an image (eg: `"https://example.com/example.png"`)
    /// - An encoded image (eg: `"data:image/svg+xml;base64,...=="`)
    pub logo: Option<String>,
    /// The contents appear at the header after the spec-url input.
    pub header: Option<String>,
    /// The contents appear at the bottom of the spec.
    pub footer: Option<String>,
    /// The contents appear at side navigation bar (only available in read-mode).
    pub nav_logo: Option<String>,
    /// The contents appear at overview section.
    /// You can link to this section using `#overview`.
    pub overview: Option<String>,
    /// The contents appear at server section.
    /// You can link to this section using `#servers`.
    pub servers: Option<String>,
    /// The contents appear at authentication section.
    /// You can link to this section using `#auth`.
    pub auth: Option<String>,
    /// contents appear at the top of all the operations but below overview > servers > auth section.
    /// Use this section to provide content that applies to all the operations.
    /// You can link this section using `#operations-top`.
    pub operations_top: Option<String>,
    /// Each tag is identified by a name, this slot can be used to insert HTML content under
    /// various tags.
    ///
    /// The first value (key) should be the name of the tag.
    /// This will be prefixed with `tag--{tag-name}` when generated.
    /// The second value (value) will be the HTML inserted.
    pub tags: HashMap<String, String>,
    /// Each path is identified by an id. The key of which is in the format of `{method}-{path}`.
    /// and certain invalid characters such as `{`, `}`, `#`, space is replaced by hyphen (-).
    /// Use this slot to insert HTML content into a specific tag.
    pub endpoints: HashMap<String, String>,
}

/// Used to control the sorting mechanism of endpoints in the rapi doc interface.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SortEndpointsBy {
    /// Sort the endpoints lexicographically by uri.
    Path,
    /// Sort the endpoints by method (e.g. `POST`, `PUT`, `TRACE`).
    Method,
}

/// Used to control the theme of the rapi doc interface.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Theme {
    /// Use a light theme.
    Light,
    /// Use a dark theme.
    Dark,
}

/// Used to contol the font size of text in the rapi doc interface.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum FontSize {
    /// Use the browsers default font size.
    Default,
    /// Make the font size larger.
    Large,
    /// Make the font size even larger.
    Largest,
}

/// Shows API Method names in the navigation bar (`GET`, `POST`, ...)
/// (if you customized nav-background make sure there is a proper contrast).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ShowMethodInNavBar {
    // `false` in RapiDocs
    /// Do not show the API Method.
    None,
    /// Show API Method as plain text.
    AsPlainText,
    /// Show API Method as colored text.
    AsColoredText,
    /// Show API Method as colored block.
    AsColoredBlock,
}

/// Navigation active item indicator styles.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum NavActiveItemMarker {
    /// Show bar on left side of active item.
    LeftBar,
    /// Highlight whole block of active item.
    ColoredBlock,
}

/// Controls navigation item spacing
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum NavItemSpacing {
    /// The standard spacing.
    Default,
    /// A more compact representation.
    Compact,
    /// Wider spacing.
    Relaxed,
}

/// Layout helps in placement of request/response sections. In column layout, request & response
/// sections are placed one below the other, In row layout they are placed side by side. This
/// attribute is applicable only when the device width is more than 768px and the render-style is
/// 'view'.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Layout {
    /// Use a row based layout.
    Row,
    /// Use a column based layout.
    Column,
}

/// Determines display of api-docs. Currently there are two modes supported. 'read' - more suitable
/// for reading and 'view' more friendly for quick exploring
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum RenderStyle {
    /// Friendly for quick exploring (expand/collapse the section of your interest).
    View,
    /// Suitable for reading (like a continuous web-page).
    Read,
    /// Similar to `read` but focuses on a single endpoint at a time (good for large specs).
    Focused,
}

/// Applies only to focused render-style. It determines the behavior of clicking on a Tag in
/// navigation bar. It can either expand-collapse the tag or take you to the tag's description page.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum NavTagClick {
    /// Expand collapsed tags when clicked.
    ExpandCollapse,
    /// Takes you to the tag's description page.
    ShowDescription,
}

impl std::fmt::Display for NavTagClick {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use NavTagClick::*;
        write!(
            fmt,
            "{}",
            match self {
                ExpandCollapse => "expand-collapse",
                ShowDescription => "show-description",
            }
        )
    }
}

/// Two different ways to display object-schemas in the responses and request bodies.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SchemaStyle {
    /// Tree based style.
    Tree,
    /// Table based style.
    Table,
}

/// Read-only fields in request schemas is always hidden but are shown in response.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SchemaHideReadOnly {
    /// Always hide read-only fields.
    Always,
    /// Never hide read-only fields.
    Never,
    /// Only hide read-only fields for `POST` methods.
    Post,
    /// Only hide read-only fields for `PUT` methods.
    Put,
    /// Only hide read-only fields for `PATCH` methods.
    Patch,
    /// Only hide read-only fields for `POST` and `PUT` methods.
    PostPut,
    /// Only hide read-only fields for `POST` and `PATCH` methods.
    PostPatch,
    /// Only hide read-only fields for `PUT` and `PATCH` methods.
    PutPatch,
    /// Only hide read-only fields for `POST`, `PUT` and `PATCH` methods.
    PostPutPatch,
}

impl std::fmt::Display for SchemaHideReadOnly {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SchemaHideReadOnly::*;
        write!(
            fmt,
            "{}",
            match self {
                Always => "always",
                Never => "never",
                Post => "post",
                Put => "put",
                Patch => "patch",
                PostPut => "post put",
                PostPatch => "post patch",
                PutPatch => "put patch",
                PostPutPatch => "post put patch",
            }
        )
    }
}

/// Read-only fields in request schemas is always hidden but are shown in response.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SchemaHideWriteOnly {
    /// Always hide read-only fields.
    Always,
    /// Never hide read-only fields.
    Never,
}

/// The schemas are displayed in two tabs - Model and Example. This option allows you to pick the
/// default tab that you would like to be active.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DefaultSchemaTab {
    /// Display the model by default.
    Model,
    /// Display the example by default.
    Example,
}

/// Determines how you want to send the api-key.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ApiKeyLocation {
    /// Send the Api Key in the header of the request.
    Header,
    /// Send the Api Key in the query params of the request.
    Query,
}

/// A RequestCredentials dictionary value indicating whether the user agent should send cookies
/// from the other domain in the case of cross-origin requests.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum FetchCredentials {
    /// Never send or receive cookies.
    Omit,
    /// Send user credentials (cookies, basic http auth, etc..)
    /// if the URL is on the same origin as the calling script.
    SameOrigin,
    /// Always send user credentials (cookies, basic http auth, etc..), even for cross-origin calls.
    Include,
}

impl std::fmt::Display for FetchCredentials {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use FetchCredentials::*;
        write!(
            fmt,
            "{}",
            match self {
                Omit => "omit",
                SameOrigin => "same-origin",
                Include => "include",
            }
        )
    }
}

macro_rules! impl_display {
    ($to_impl:ident) => {
        impl std::fmt::Display for $to_impl {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                let dbg_repr = format!("{:?}", self);
                write!(fmt, "{}", dbg_repr.to_lowercase())
            }
        }
    };
}

impl_display!(SortEndpointsBy);
impl_display!(Theme);
impl_display!(FontSize);
impl_display!(ShowMethodInNavBar);
impl_display!(NavActiveItemMarker);
impl_display!(NavItemSpacing);
impl_display!(Layout);
impl_display!(RenderStyle);
impl_display!(SchemaStyle);
impl_display!(SchemaHideWriteOnly);
impl_display!(DefaultSchemaTab);
impl_display!(ApiKeyLocation);

fn slot_list(slots: &[String]) -> String {
    let mut result = "".to_owned();
    for html in slots {
        // Append new slot
        result = format!(r#"{}<slot>{}</slot>"#, result, html);
    }
    result
}

fn slot_opt(slot: &Option<String>, name: &str) -> String {
    match slot {
        Some(html) => format!(r#"<slot name="{}">{}</slot>"#, name, html),
        None => "".to_owned(),
    }
}

fn slot_logo(slot: &Option<String>) -> String {
    match slot {
        Some(html) => format!(
            r#"<img slot="logo" src="{}" alt="logo" style="max-width: 150px; max-height: 50px"/>"#,
            html
        ),
        None => "".to_owned(),
    }
}

fn slot_tags(slots: &HashMap<String, String>) -> String {
    let mut result = "".to_owned();
    for (key, html) in slots {
        // Append new slot
        result = format!(r#"{}<slot name="tag--{}">{}</slot>"#, result, key, html);
    }
    result
}

fn slot_endpoints(slots: &HashMap<String, String>) -> String {
    let mut result = "".to_owned();
    for (key, html) in slots {
        if key.contains('{') || key.contains('}') || key.contains('#') || key.contains(' ') {
            if cfg!(debug_assertions) {
                panic!(
                    "Slot endpoint `{}` contains invalid characters `{{`, `}}`, `#` or ` ` (space).",
                    key
                );
            } else {
                eprintln!(
                    "Slot endpoint `{}` contains invalid characters `{{`, `}}`, `#` or ` ` (space).",
                    key
                );
            }
        }
        // Append new slot
        result = format!(r#"{}<slot name="{}">{}</slot>"#, result, key, html);
    }
    result
}

/// Transform the provided `RapiDocConfig` into a list of `Route`s that serve the RapiDoc ui.
pub fn make_rapidoc(config: &RapiDocConfig) -> impl Into<Vec<Route>> {
    let title = match &config.title {
        Some(title) => title.clone(),
        None => "API Documentation | RapiDoc".to_owned(),
    };
    let template_map = hash_map! {
        // General
        "TITLE" => title,
        "SPEC_URL" => config.general.spec_urls[0].url.clone(),
        // Can be used for custom html files
        "SPEC_URLS" => serde_json::to_string(&config.general.spec_urls).unwrap_or_default(),
        "UPDATE_ROUTE" => config.general.update_route.to_string(),
        "ROUTE_PREFIX" => config.general.route_prefix.clone(),
        "SORT_TAGS" => config.general.sort_tags.to_string(),
        "SORT_ENDPOINTS_BY" => config.general.sort_endpoints_by.to_string(),
        "HEADING_TEXT" => config.general.heading_text.clone(),
        "GOTO_PATH" => config.general.goto_path.clone(),
        "REQUEST_EXAMPLE_FIELDS" => config.general.fill_request_fields_with_example.to_string(),
        "PERSIST_AUTH" => config.general.persist_auth.to_string(),
        // UI Colors and Fonts
        "THEME" => config.ui.theme.to_string(),
        "BG_COLOR" => config.ui.bg_color.clone(),
        "TEXT_COLOR" => config.ui.text_color.clone(),
        "HEADER_COLOR" => config.ui.header_color.clone(),
        "PRIMARY_COLOR" => config.ui.primary_color.clone(),
        "LOAD_FONTS" => config.ui.load_fonts.to_string(),
        "REGULAR_FONT" => config.ui.regular_font.clone(),
        "MONO_FONT" => config.ui.mono_font.clone(),
        "FONT_SIZE" => config.ui.font_size.to_string(),
        "CSS_FILE" => config.ui.css_file.clone().unwrap_or_default(),
        "CSS_CLASSES" => config.ui.css_classes.join(" ").to_string(),
        // Navigation bar settings
        "SHOW_METHOD_IN_NAV_BAR" => config.nav.show_method_in_nav_bar.to_string(),
        "USE_PATH_IN_NAV_BAR" => config.nav.use_path_in_nav_bar.to_string(),
        "NAV_BG_COLOR" => config.nav.nav_bg_color.clone(),
        "NAV_TEXT_COLOR" => config.nav.nav_text_color.clone(),
        "NAV_HOVER_BG_COLOR" => config.nav.nav_hover_bg_color.clone(),
        "NAV_HOVER_TEXT_COLOR" => config.nav.nav_hover_text_color.clone(),
        "NAV_ACCENT_COLOR" => config.nav.nav_accent_color.clone(),
        "NAV_ACCENT_TEXT_COLOR" => config.nav.nav_accent_text_color.clone(),
        "NAV_ACCENT_ITEM_MARKER" => config.nav.nav_active_item_marker.to_string(),
        "NAV_ITEM_SPACING" => config.nav.nav_item_spacing.to_string(),
        "ON_NAV_TAG_CLICK" => config.nav.on_nav_tag_click.to_string(),
        // UI Layout & Placement
        "LAYOUT" => config.layout.layout.to_string(),
        "RENDER_STYLE" => config.layout.render_style.to_string(),
        "RESPONSE_AREA_HEIGHT" => config.layout.response_area_height.clone(),
        // Hide/Show Sections
        "SHOW_INFO" => config.hide_show.show_info.to_string(),
        "INFO_DESCRIPTIONS_IN_NAVBAR" => config.hide_show.info_description_headings_in_navbar.to_string(),
        "SHOW_COMPONENTS" => config.hide_show.show_components.to_string(),
        "SHOW_HEADER" => config.hide_show.show_header.to_string(),
        "ALLOW_AUTHENTICATION" => config.hide_show.allow_authentication.to_string(),
        "ALLOW_SPEC_URL_LOAD" => config.hide_show.allow_spec_url_load.to_string(),
        "ALLOW_SPEC_FILE_LOAD" => config.hide_show.allow_spec_file_load.to_string(),
        "ALLOW_SPEC_FILE_DOWNLOAD" => config.hide_show.allow_spec_file_download.to_string(),
        "ALLOW_SEARCH" => config.hide_show.allow_search.to_string(),
        "ALLOW_ADVANCED_SEARCH" => config.hide_show.allow_advanced_search.to_string(),
        "ALLOW_TRY" => config.hide_show.allow_try.to_string(),
        "SHOW_CURL_BEFORE_TRY" => config.hide_show.show_curl_before_try.to_string(),
        "ALLOW_SERVER_SELECTION" => config.hide_show.allow_server_selection.to_string(),
        "ALLOW_SCHEMA_DESC_EXPAND_TOGGLE" => config.hide_show.allow_schema_description_expand_toggle.to_string(),
        // Schema Sections
        "SCHEMA_STYLE" => config.schema.schema_style.to_string(),
        "SCHEMA_EXPAND_LEVEL" => config.schema.schema_expand_level.to_string(),
        "SCHEMA_DESCRIPTION_EXPANDED" => config.schema.schema_description_expanded.to_string(),
        "SCHEMA_HIDE_READ_ONLY" => config.schema.schema_hide_read_only.to_string(),
        "SCHEMA_HIDE_WRITE_ONLY" => config.schema.schema_hide_write_only.to_string(),
        "DEFAULT_SCHEMA_TAB" => config.schema.default_schema_tab.to_string(),
        // API Server & calls
        "SERVER_URL" => config.api.server_url.clone(),
        "DEFAULT_API_SERVER" => config.api.default_api_server.clone(),
        "API_KEY_NAME" => config.api.api_key_name.clone(),
        "API_KEY_LOCATION" => config.api.api_key_location.as_ref().map_or_else(|| "".to_owned(), |v| v.to_string()),
        "API_KEY_VALUE" => config.api.api_key_value.clone(),
        "FETCH_CREDENTIALS" => config.api.fetch_credentials.as_ref().map_or_else(|| "".to_owned(), |v| v.to_string()),
        // Slots
        "DEFAULT" => slot_list(&config.slots.default),
        "LOGO" => slot_logo(&config.slots.logo),
        "HEADER" => slot_opt(&config.slots.header, "header"),
        "FOOTER" => slot_opt(&config.slots.footer, "footer"),
        "NAV_LOGO" => slot_opt(&config.slots.nav_logo, "nav-logo"),
        "OVERVIEW" => slot_opt(&config.slots.overview, "overview"),
        "SERVERS" => slot_opt(&config.slots.servers, "servers"),
        "AUTH" => slot_opt(&config.slots.auth, "auth"),
        "OPERATIONS_TOP" => slot_opt(&config.slots.operations_top, "operations-top"),
        "TAGS" => slot_tags(&config.slots.tags),
        "ENDPOINTS" => slot_endpoints(&config.slots.tags),
    };

    let mut index_page = match &config.custom_html {
        Some(custom_file) => custom_file.clone(),
        None => include_str!("../rapidoc/index.html").to_owned(),
    };
    // Replace custom tags
    for (key, value) in &config.custom_template_tags {
        // Replace `{{KEY}}` with `VALUE`, So `{{ {{ KEY }} }}` => `{ { KEY } }`
        index_page = index_page.replace(&format!("{{{{{}}}}}", key), value);
    }
    for (key, value) in template_map {
        // Replace `{{KEY}}` with `VALUE`, So `{{ {{ KEY }} }}` => `{ { KEY } }`
        index_page = index_page.replace(&format!("{{{{{}}}}}", key), &value);
    }

    vec![
        RedirectHandler::to("index.html").into_route("/"),
        // Add custom html file
        ContentHandler::bytes_owned(ContentType::HTML, index_page.as_bytes().to_vec())
            .into_route("/index.html"),
        // Add other static files
        static_file!("rapidoc-min.js", JavaScript),
        static_file!("oauth-receiver.html", HTML),
    ]
}

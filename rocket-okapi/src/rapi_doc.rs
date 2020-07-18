use crate::swagger_ui::UrlObject;
use crate::handlers::{ContentHandler, RedirectHandler};

use rocket::http::ContentType;
use rocket::Route;

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone, Default)]
pub struct RapiDocConfig {
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
    /// Settings used to confugre access to the api.
    pub api: ApiConfig,
    /// Settings to configure the Rapi Doc "slots"
    pub slots: SlotsConfig,
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone)]
pub struct GeneralConfig {
    /// Urls of the OpenAPI spec to view.
    ///
    /// This field _must_ be manually filled with at least one element.
    pub spec_urls: Vec<UrlObject>,
    /// To list tags in alphabetic order, otherwise tags will be ordered based on how it is
    /// specified under the tags section in the spec.
    ///
    /// The default is `false`.
    pub sort_tags: bool,
    /// Sort endpoints within each tags by path or method.
    ///
    /// The default is `SortEndpointsBy::Path`.
    pub sort_endpoints_by: SortEndpointsBy,
    /// Heading Text on top-left corner.
    pub heading_text: String,
    /// Initial location on the document(identified by method and path) where you want to go after
    /// the spec is loaded. `goto_path` should be in the form of {method}-{path}. For instance you
    /// want to scrollTo GET /user/login you should provide the location as get-/user/login.
    pub goto_path: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            spec_urls: vec![],
            sort_tags: false,
            sort_endpoints_by: SortEndpointsBy::Path,
            heading_text: "".to_string(),
            goto_path: "".to_string(),
        }
    }
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone)]
pub struct UiConfig {
    /// Is the base theme, which is used for calculating colors for various UI components. 'theme',
    /// 'bg-color' and 'text-color' are the base attributes for generating a custom theme.
    ///
    /// The default is `Theme::Light`.
    pub theme: Theme,
    /// Hex color code for main background.
    pub bg_color: String,
    /// Hex color code for text.
    pub text_color: String,
    /// Hex color code for the header's background.
    pub header_color: String,
    /// Hex color code on various controls such as buttons, tabs.
    pub primary_color: String,
    /// Font Name(s) to be used for regular text.
    pub regular_font: String,
    /// Font Name(s) to be used for mono-spaced text.
    pub mono_font: String,
    /// Sets the relative font sizes for the entire document.
    ///
    /// The default is `FontSize::Default`.
    pub font_size: FontSize,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            bg_color: "".to_string(),
            text_color: "".to_string(),
            header_color: "".to_string(),
            primary_color: "".to_string(),
            regular_font: "".to_string(),
            mono_font: "".to_string(),
            font_size: FontSize::Default,
        }
    }
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone)]
pub struct NavConfig {
    /// Navigation bar's background color.
    pub nav_bg_color: String,
    /// URL of navigation bar's background image.
    pub nav_bg_image: String,
    /// Navigation bar's background image size (same as css background-size property) allowed values
    /// are. 
    ///
    /// The default is `NavBgImageSize::Auto`.
    pub nav_bg_image_size: NavBgImageSize,
    /// Navigation bar's background image repeat (same as css background-repeat property) allowed
    /// values are.
    ///
    /// The default is `NavBgImageSize::Repeat`.
    pub nav_bg_image_repeat: NavBgImageRepeat,
    /// Navigation bar's Text color.
    pub nav_text_color: String,
    /// Background color of the navigation item on mouse-over.
    pub nav_hover_bg_color: String,
    /// Text color of the navigation item on mouse-over.
    pub nav_hover_text_color: String,
    /// Current selected item indicator.
    pub nav_accent_color: String,
    /// Controls navigation item spacing.
    ///
    /// The default is `NavItemSpacing::Default`.
    pub nav_item_spacing: NavItemSpacing,
}

impl Default for NavConfig {
    fn default() -> Self {
        Self {
            nav_bg_color: "".to_string(),
            nav_bg_image: "".to_string(),
            nav_bg_image_size: NavBgImageSize::Auto,
            nav_bg_image_repeat: NavBgImageRepeat::Repeat,
            nav_text_color: "".to_string(),
            nav_hover_bg_color: "".to_string(),
            nav_hover_text_color: "".to_string(),
            nav_accent_color: "".to_string(),
            nav_item_spacing: NavItemSpacing::Default,
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
    /// Determines display of api-docs. Currently there are two modes supported. 'read' - more
    /// suitable for reading and 'view' more friendly for quick exploring.
    ///
    /// The default is `RenderStyle::View`.
    pub render_style: RenderStyle,
    /// Two different ways to display object-schemas in the responses and request bodies.
    ///
    /// The default is `SchemaStyle::Tree`.
    pub schema_style: SchemaStyle,
    /// Schemas are expanded by default, use this attribute to control how many levels in the schema
    /// should be expanded.
    ///
    /// The default is `999`.
    pub schema_expand_level: usize,
    /// Constraint and descriptions information of fields in the schema are collapsed to show only
    /// the first line. Set it to true if you want them to fully expanded.
    ///
    /// The default is `false`.
    pub schema_description_expanded: bool,
    /// The schemas are displayed in two tabs - Model and Example. This option allows you to pick
    /// the default tab that you would like to be active.
    ///
    /// The default is `DefaultSchemaTab::Model`.
    pub default_schema_tab: DefaultSchemaTab,
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
            render_style: RenderStyle::View,
            schema_style: SchemaStyle::Tree,
            schema_expand_level: 999,
            schema_description_expanded: false,
            default_schema_tab: DefaultSchemaTab::Model,
            response_area_height: "300px".to_string(),
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
    /// If set to `false`, user will not be able to search APIs.
    ///
    /// The default is `true`.
    pub allow_search: bool,
    /// 'TRY' feature allows you to make REST calls to the API server. To disable this feature set
    /// it to false
    /// Setting it to false will also hide API-Servers if specified in the spec.
    ///
    /// The default is `true`.
    pub allow_try: bool,
    /// If set to 'false', user will not be able to see or select API server (Server List will be
    /// hidden, however users will be able to see the server url near the 'TRY' button, to know in
    /// advance where the TRY will send the request). The URL specified in the server-url attribute
    /// will be used if set, else the first server in the API specification file will be used.
    ///
    /// The default is `true`.
    pub allow_server_selection: bool,
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
            allow_search: true,
            allow_try: true,
            allow_server_selection: true,
        }
    }
}

/// Used to configure api access.
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Name of the API key that will be send while trying out the APIs.
    ///
    /// The default is "Authorization".
    pub api_key_name: String,
    /// Value of the API key that will be send while trying out the APIs. This can also be
    /// provided/overwritten from UI. 
    pub api_key_value: String,
    /// Determines how you want to send the api-key.
    ///
    /// The default is `ApiKeyLocation::Header`.
    pub api_key_location: ApiKeyLocation,
    /// OpenAPI spec has a provision for providing the server url. The UI will list all the server
    /// URLs provided in the spec. The user can then select one URL to which he or she intends to
    /// send API calls while trying out the apis. However, if you want to provide an API server of
    /// your own which is not listed in the spec, you can use this property to provide one. It is
    /// helpful in the cases where the same spec is shared between multiple environment say Dev and
    /// Test and each have their own API server. 
    pub server_url: String,
    /// If you have multiple api-server listed in the spec, use this attribute to select the default
    /// API server, where all the API calls will goto. This can be changed later from the UI 
    pub default_api_server: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            api_key_name: "".to_string(),
            api_key_value: "".to_string(),
            api_key_location: ApiKeyLocation::Header,
            server_url: "".to_string(),
            default_api_server: "".to_string(),
        }
    }
}

/// Config used to configure the slots.
#[derive(Debug, Clone, Default)]
pub struct SlotsConfig {
    /// The url to the logo.
    pub logo: Option<String>,
    // /// Text to place in the header
    // pub header: Option<String>,
    /// Text to place in the footer.
    pub footer: Option<String>,
    /// The url to the logo in the nav bar.
    pub nav_logo: Option<String>,
}

/// Used to control the sorting mechanism of endpoints in the rapi doc interface.
#[derive(Debug, Clone)]
pub enum SortEndpointsBy {
    /// Sort the endpoints lexicographically by uri.
    Path,
    /// Sort the endpoints by method (e.g. `POST`, `PUT`, `TRACE`).
    Method,
}

/// Used to control the theme of the rapi doc interface.
#[derive(Debug, Clone)]
pub enum Theme {
    /// Use a light theme.
    Light,
    /// Use a dark theme.
    Dark,
}

/// Used to contol the font size of text in the rapi doc interface.
#[derive(Debug, Clone)]
pub enum FontSize {
    /// Use the browsers default font size.
    Default,
    /// Make the font size larger.
    Large,
    /// Make the font size even larger.
    Largest,
}

/// Used to control the size of the background image in the nav bar.
#[derive(Debug, Clone)]
pub enum NavBgImageSize {
    /// Default value. The background image is displayed in its original size.
    Auto,
    /// Sets the width and height of the background image. The first value sets the width, the
    /// second value sets the height. If only one value is given, the second is set to "auto".
    Length,
    /// Resize the background image to cover the entire container, even if it has to stretch the
    /// image or cut a little bit off one of the edges.
    Cover,
    /// Resize the background image to make sure the image is fully visible.
    Contain,
    /// Sets this property to its default value.
    Initial,
    /// Inherits this property from its parent element.
    Inherit,
}

/// Used to control the repeating of the background image in the nav bar.
#[derive(Debug, Clone)]
pub enum NavBgImageRepeat {
    /// The background image is repeated both vertically and horizontally.  The last image will be
    /// clipped if it does not fit. This is default.
    Repeat,
    /// The background image is repeated only horizontally.
    RepeatX,
    /// The background image is repeated only vertically.
    RepeatY,
    /// The background-image is not repeated. The image will only be shown once.
    NoRepeat,
    /// Sets this property to its default value.
    Initial,
    /// Inherits this property from its parent element.
    Inherit,
}

/// Controls navigation item spacing 
#[derive(Debug, Clone)]
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
pub enum Layout {
    /// Use a row based layout.
    Row,
    /// Use a column based layout.
    Column,
}

/// Determines display of api-docs. Currently there are two modes supported. 'read' - more suitable
/// for reading and 'view' more friendly for quick exploring 
#[derive(Debug, Clone)]
pub enum RenderStyle {
    /// Read-first layout.
    Read,
    /// View-first layout.
    View,
}

/// Two different ways to display object-schemas in the responses and request bodies.
#[derive(Debug, Clone)]
pub enum SchemaStyle {
    /// Tree based style.
    Tree,
    /// Table based style.
    Table,
}

/// The schemas are displayed in two tabs - Model and Example. This option allows you to pick the
/// default tab that you would like to be active.
#[derive(Debug, Clone)]
pub enum DefaultSchemaTab {
    /// Display the model by default.
    Model,
    /// Display the example by default.
    Example,
}

/// Determines how you want to send the api-key. 
#[derive(Debug, Clone)]
pub enum ApiKeyLocation {
    /// Send the Api Key in the header of the request.
    Header,
    /// Send the Api Key in the query params of the request.
    Query,
}

macro_rules! impl_to_str_for {
    ($to_impl:ident) => {
        impl std::fmt::Display for $to_impl {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                let dbg_repr = format!("{:?}", self);
                write!(fmt, "{}", dbg_repr.to_lowercase())
            }
        }
    };
}

impl_to_str_for!(SortEndpointsBy);
impl_to_str_for!(Theme);
impl_to_str_for!(FontSize);
impl_to_str_for!(NavBgImageRepeat);
impl_to_str_for!(NavItemSpacing);
impl_to_str_for!(Layout);
impl_to_str_for!(RenderStyle);
impl_to_str_for!(SchemaStyle);
impl_to_str_for!(DefaultSchemaTab);
impl_to_str_for!(ApiKeyLocation);

fn pad<S: std::fmt::Display>(s: &S) -> String {
    format!("\"{}\"", s)
}

/// Transform the provided `RapiDocConfig` into a list of `Route`s that serve the rapi docs ui.
pub fn make_rapi_doc(config: &RapiDocConfig) -> impl Into<Vec<Route>> {
    let rapi_content = include_str!("../rapi-doc/rapi-doc.html");
    let options = config
        .general
        .spec_urls
        .iter()
        .map(|su| format!("<option value=\"{}\">{}</option>", su.url, su.name))
        .collect::<Vec<_>>()
        .join("\n         ");
    let logo = config.slots.logo.as_ref().map(|l| format!(r#"<img slot="logo" src="{}" width="110px" height="30px"/>"#, l)).unwrap_or_default();
    let nav_logo = config.slots.nav_logo.as_ref().map(|l| format!(r#"<img slot="nav-logo" src="{}" width="110px" height="30px"/>"#, l)).unwrap_or_default();
    let footer = config.slots.footer.as_ref().map(|f| format!(r#"<p slot="footer">{}</p>"#, f)).unwrap_or_default();
    let rapi_content = rapi_content
        .replace("ROUTES", &options)
        .replace("\"SPEC_URL\"", &pad(&config.general.spec_urls[0].url))
        .replace("\"SORT_TAGS\"", &pad(&config.general.sort_tags))
        .replace("\"SORT_ENDPOINTS_BY\"", &pad(&config.general.sort_endpoints_by))
        .replace("\"HEADING_TEXT\"", &pad(&config.general.heading_text))
        .replace("\"GOTO_PATH\"", &pad(&config.general.goto_path))
        .replace("\"THEME\"", &pad(&config.ui.theme))
        .replace("\"BG_COLOR\"", &pad(&config.ui.bg_color))
        .replace("\"TEXT_COLOR\"", &pad(&config.ui.text_color))
        .replace("\"HEADER_COLOR\"", &pad(&config.ui.header_color))
        .replace("\"PRIMARY_COLOR\"", &pad(&config.ui.primary_color))
        .replace("\"REGULAR_FONT\"", &pad(&config.ui.regular_font))
        .replace("\"MONO_FONT\"", &pad(&config.ui.mono_font))
        .replace("\"FONT_SIZE\"", &pad(&config.ui.font_size))
        .replace("\"NAV_BG_COLOR\"", &pad(&config.nav.nav_bg_color))
        .replace("\"NAV_BG_IMAGE\"", &pad(&config.nav.nav_bg_image))
        .replace("\"NAV_BG_IMAGE_REPEAT\"", &pad(&config.nav.nav_bg_image_repeat))
        .replace("\"NAV_TEXT_COLOR\"", &pad(&config.nav.nav_text_color))
        .replace("\"NAV_HOVER_BG_COLOR\"", &pad(&config.nav.nav_hover_bg_color))
        .replace("\"NAV_HOVER_TEXT_COLOR\"", &pad(&config.nav.nav_hover_text_color))
        .replace("\"NAV_ACCENT_COLOR\"", &pad(&config.nav.nav_accent_color))
        .replace("\"NAV_ITEM_SPACING\"", &pad(&config.nav.nav_item_spacing))
        .replace("\"LAYOUT\"", &pad(&config.layout.layout))
        .replace("\"RENDER_STYLE\"", &pad(&config.layout.render_style))
        .replace("\"SCHEMA_STYLE\"", &pad(&config.layout.schema_style))
        .replace("\"SCHEMA_EXPAND_LEVEL\"", &pad(&config.layout.schema_expand_level))
        .replace("\"SCHEMA_DESCRIPTION_EXPANDED\"", &pad(&config.layout.schema_description_expanded))
        .replace("\"DEFAULT_SCHEMA_TAB\"", &pad(&config.layout.default_schema_tab))
        .replace("\"RESPONSE_AREA_HEIGHT\"", &pad(&config.layout.response_area_height))
        .replace("\"SHOW_INFO\"", &pad(&config.hide_show.show_info))
        .replace("\"INFO_DESCRIPTIONS_IN_NAVBAR\"", &pad(&config.hide_show.info_description_headings_in_navbar))
        .replace("\"SHOW_COMPONENTS\"", &pad(&config.hide_show.show_components))
        .replace("\"SHOW_HEADER\"", &pad(&config.hide_show.show_header))
        .replace("\"ALLOW_AUTHENTICATION\"", &pad(&config.hide_show.allow_authentication))
        .replace("\"ALLOW_SPEC_URL_LOAD\"", &pad(&config.hide_show.allow_spec_url_load))
        .replace("\"ALLOW_SPEC_FILE_LOAD\"", &pad(&config.hide_show.allow_spec_file_load))
        .replace("\"ALLOW_SEARCH\"", &pad(&config.hide_show.allow_search))
        .replace("\"ALLOW_TRY\"", &pad(&config.hide_show.allow_try))
        .replace("\"ALLOW_SERVER_SELECTION\"", &pad(&config.hide_show.allow_server_selection))
        .replace("\"API_KEY_NAME\"", &pad(&config.api.api_key_name))
        .replace("\"API_KEY_VALUE\"", &pad(&config.api.api_key_value))
        .replace("\"API_KEY_LOCATION\"", &pad(&config.api.api_key_location))
        .replace("\"SERVER_URL\"", &pad(&config.api.server_url))
        .replace("\"DEFAULT_API_SERVER\"", &pad(&config.api.default_api_server))
        .replace("<LOGO/>", &logo)
        .replace("<FOOTER/>", &footer)
        .replace("<NAV_LOGO/>", &nav_logo);

    vec![
        RedirectHandler::to("rapi-doc.html").into_route("/"),
        ContentHandler::bytes_owned(
            ContentType::HTML,
            rapi_content.as_bytes().to_vec(),
        )
        .into_route(concat!("/", "/"))
    ]
}

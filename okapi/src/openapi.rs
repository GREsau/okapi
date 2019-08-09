use crate::Map;
use schemars::schema::{Ref, SchemaObject};
use schemars::MakeSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
type Object = Map<String, Value>;

type SecurityRequirement = Map<String, Vec<String>>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(untagged)]
pub enum RefOr<T> {
    Ref(Ref),
    Object(T),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenAPI {
    pub openapi: String,
    pub info: Info,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
    pub paths: Map<String, PathItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<SecurityRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    pub version: String,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, MakeSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct Contact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub variables: Map<String, ServerVariable>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {
    #[serde(default, rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<String>>,
    pub default: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, MakeSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct PathItem {
    #[serde(default, rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pust: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RefOr<Parameter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RefOr<RequestBody>>,
    pub responses: Responses,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub callbacks: Map<String, RefOr<Callback>>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, MakeSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct Responses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<RefOr<Response>>,
    #[serde(flatten)]
    pub responses: Map<String, RefOr<Response>>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, MakeSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct Components {
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub schemas: Map<String, RefOr<SchemaObject>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub responses: Map<String, RefOr<Response>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub parameters: Map<String, RefOr<Parameter>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub examples: Map<String, RefOr<Example>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub request_bodies: Map<String, RefOr<RequestBody>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub headers: Map<String, RefOr<Header>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub security_schemes: Map<String, RefOr<SecurityScheme>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<String, RefOr<Link>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub callbacks: Map<String, RefOr<Callback>>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub description: String,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub headers: Map<String, RefOr<Header>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub content: Map<String, MediaType>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub links: Map<String, RefOr<Link>>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_empty_value: bool,
    #[serde(flatten)]
    pub value: ParameterValue,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ParameterValue {
    Schema {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<ParameterStyle>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        explode: Option<bool>,
        #[serde(default, skip_serializing_if = "is_false")]
        allow_reserved: bool,
        schema: RefOr<SchemaObject>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        examples: Option<Map<String, Example>>,
    },
    Content {
        content: Map<String, MediaType>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub enum ParameterStyle {
    Matrix,
    Label,
    Form,
    Simple,
    SpaceDelimited,
    PipeDelimited,
    DeepObject,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub value: ExampleValue,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub enum ExampleValue {
    Value(Value),
    ExternalValue(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content: Map<String, MediaType>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_empty_value: bool,
    #[serde(flatten)]
    pub value: ParameterValue,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub data: SecuritySchemeData,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SecuritySchemeData {
    ApiKey {
        name: String,
        #[serde(rename = "in")]
        location: String,
    },
    Http {
        scheme: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bearer_format: Option<String>,
    },
    #[serde(rename = "oauth2")]
    OAuth2 {
        flows: OAuthFlows,
    },
    OpenIdConnect {
        open_id_connect_url: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlow>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlow {
    pub authorization_url: String,
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub scopes: Map<String, String>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    // TODO operationRef XOR operationId must be specified
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub parameters: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_body: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Callback {
    #[serde(flatten)]
    pub callbacks: Map<String, PathItem>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<RefOr<SchemaObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Map<String, Example>>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub encoding: Map<String, Encoding>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: String,
    #[serde(flatten)]
    pub extensions: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, MakeSchema)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub headers: Map<String, RefOr<Header>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_reserved: bool,
    #[serde(flatten)]
    pub extensions: Object,
}

pub fn is_false(b: &bool) -> bool {
    !b
}

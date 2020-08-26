use super::OpenApiFromQuery;
use crate::gen::OpenApiGenerator;
use okapi::openapi3::*;
use schemars::schema::{Schema, SchemaObject};
use schemars::JsonSchema;
use std::result::Result as StdResult;

type Result = crate::Result<Vec<Parameter>>;

/// Given an object that implements the `JsonSchema` generate all the `Parameter`
/// that are used to create documentation.
/// Use when manualy implementing a
/// [Query Guard](https://docs.rs/rocket/latest/rocket/request/trait.FromQuery.html).
/// Example:
/// ```
/// use rocket::request::{Query, FromQuery};
/// use serde::{Serialize, Deserialize};
/// use schemars::JsonSchema;
/// use rocket_okapi::{
///     gen::OpenApiGenerator,
///     request::OpenApiFromQuery,
///     request::get_nested_query_parameters
/// };
///
/// #[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
/// pub struct ApiPagination{
///     page: Option<u32>,
///     per_page: Option<u32>,
/// }
///
/// impl<'q> FromQuery<'q> for ApiPagination {
///     type Error = String;// Some kind of error
///
///     fn from_query(_query: Query<'q>) -> Result<Self, Self::Error> {
///         Ok(ApiPagination::default())
///     }
/// }
///
/// impl<'r> OpenApiFromQuery<'r> for ApiPagination {
///     fn query_multi_parameter(gen: &mut OpenApiGenerator, name: String, required: bool)
///     -> rocket_okapi::Result<Vec<okapi::openapi3::Parameter>> {
///         Ok(get_nested_query_parameters::<ApiPagination>(gen, name, required))
///     }
/// }
/// ```
pub fn get_nested_query_parameters<'r, T>(
    gen: &mut OpenApiGenerator,
    _name: String,
    required: bool,
) -> Vec<Parameter>
where
    T: JsonSchema,
{
    let schema = gen.json_schema_no_ref::<T>();
    // Get a list of properties from the structure.
    let mut properties: schemars::Map<String, Schema> = schemars::Map::new();
    if let Some(object) = schema.object {
        properties = object.properties;
    }
    // Create all the `Parameter` for every property
    let mut parameter_list: Vec<Parameter> = Vec::new();
    for (key, property) in properties {
        let prop_schema = match property {
            Schema::Object(x) => x,
            _ => SchemaObject::default(),
        };
        let mut parameter_required = required;
        // Check if parameter is optional (only is not already optional)
        if parameter_required {
            for (key, value) in &prop_schema.extensions {
                if key == "nullable" {
                    if let Some(nullable) = value.as_bool() {
                        parameter_required = !nullable;
                    }
                }
            }
        }
        let description = prop_schema
            .metadata
            .as_ref()
            .and_then(|m| m.description.clone());
        parameter_list.push(Parameter {
            name: key,
            location: "query".to_owned(),
            description,
            required: parameter_required,
            deprecated: false,
            allow_empty_value: false,
            value: ParameterValue::Schema {
                style: None,
                explode: None,
                allow_reserved: false,
                schema: prop_schema,
                example: None,
                examples: None,
            },
            extensions: Default::default(),
        });
    }
    return parameter_list;
}

impl<'r, T: OpenApiFromQuery<'r>> OpenApiFromQuery<'r> for StdResult<T, T::Error> {
    fn query_multi_parameter(gen: &mut OpenApiGenerator, name: String, _required: bool) -> Result {
        T::query_multi_parameter(gen, name, false)
    }
}

impl<'r, T: OpenApiFromQuery<'r>> OpenApiFromQuery<'r> for Option<T> {
    fn query_multi_parameter(gen: &mut OpenApiGenerator, name: String, _required: bool) -> Result {
        T::query_multi_parameter(gen, name, false)
    }
}

// All fields are required.
// Does not allow extra fields.
impl<'r, T> OpenApiFromQuery<'r> for rocket::request::Form<T>
where
    T: rocket::request::FromForm<'r> + JsonSchema,
{
    fn query_multi_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        Ok(get_nested_query_parameters::<T>(gen, name, required))
    }
}

// All fields are required.
// Does allow extra fields. (automatically discards extra fields without error)
impl<'r, T> OpenApiFromQuery<'r> for rocket::request::LenientForm<T>
where
    T: rocket::request::FromForm<'r> + JsonSchema,
{
    fn query_multi_parameter(gen: &mut OpenApiGenerator, name: String, required: bool) -> Result {
        Ok(get_nested_query_parameters::<T>(gen, name, required))
    }
}

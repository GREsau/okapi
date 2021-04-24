// use super::OpenApiFromQuery;
use crate::gen::OpenApiGenerator;
use okapi::openapi3::*;
use schemars::schema::{Schema, SchemaObject};
use schemars::JsonSchema;

/// Given an object that implements the `JsonSchema` generate all the `Parameter` that are used to
/// create documentation. Use when manualy implementing a
/// [Query Guard](https://docs.rs/rocket/latest/rocket/request/trait.FromQuery.html).
/// ### Example
/// ```ignore
/// use rocket::form::FromForm;
/// use serde::{Serialize, Deserialize};
/// use schemars::JsonSchema;
/// use rocket_okapi::{
///     gen::OpenApiGenerator,
///     request::OpenApiFromForm,
///     request::get_nested_form_parameters
/// };
///
/// #[derive(Serialize, Deserialize, Clone, Debug, Default)]
/// pub struct ApiPagination{
///     page: Option<u32>,
///     per_page: Option<u32>,
/// }
///
/// impl<'q> FromForm<'q> for ApiPagination {
///     
/// }
///
/// impl<'r> OpenApiFromForm<'r> for ApiPagination {
///     fn form_multi_parameter(gen: &mut OpenApiGenerator, name: String, required: bool)
///     -> rocket_okapi::Result<Vec<okapi::openapi3::Parameter>> {
///         Ok(get_nested_form_parameters::<ApiPagination>(gen, name, required))
///     }
/// }
/// ```
pub(crate) fn get_nested_form_parameters<T>(
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
            .as_deref()
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
    parameter_list
}

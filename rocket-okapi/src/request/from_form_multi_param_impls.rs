use crate::gen::OpenApiGenerator;
use okapi::openapi3::{Parameter, ParameterValue};
use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};
use schemars::JsonSchema;

/// Given an object that implements the `JsonSchema` generate all the `Parameter`
/// that are used to create documentation.
/// Use when manually implementing a
/// [Form Guard](https://api.rocket.rs/master/rocket/form/trait.FromForm.html).
/// Example:
/// ```
/// use rocket::form::FromForm;
/// use serde::{Serialize, Deserialize};
/// use schemars::JsonSchema;
/// use rocket_okapi::{
///     gen::OpenApiGenerator,
///     request::OpenApiFromForm,
///     request::get_nested_form_parameters
/// };
///
/// #[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema, FromForm)]
/// pub struct ApiPagination{
///     page: Option<u32>,
///     per_page: Option<u32>,
/// }
pub fn get_nested_form_parameters<T>(
    gen: &mut OpenApiGenerator,
    name: String,
    required: bool,
) -> Vec<Parameter>
where
    T: JsonSchema,
{
    let schema = gen.json_schema_no_ref::<T>();
    // Get a list of properties from the structure.
    let mut properties: schemars::Map<String, Schema> = schemars::Map::new();
    // Create all the `Parameter` for every property
    let mut parameter_list: Vec<Parameter> = Vec::new();
    match &schema.instance_type {
        Some(SingleOrVec::Single(instance_type)) => {
            if **instance_type == InstanceType::Object {
                if let Some(object) = schema.object {
                    properties = object.properties;
                }
                for (key, property) in properties {
                    let prop_schema = match property {
                        Schema::Object(x) => x,
                        _ => SchemaObject::default(),
                    };
                    parameter_list.push(parameter_from_schema(prop_schema, key, required));
                }
            } else {
                parameter_list.push(parameter_from_schema(schema, name, required));
            }
        }
        _ => {
            // TODO: Do nothing for now, might need implementation later.
            log::warn!(
                "Please let `okapi` devs know how you triggered this type: `{:?}`.",
                schema.instance_type
            );
        }
    }
    parameter_list
}

fn parameter_from_schema(schema: SchemaObject, name: String, mut required: bool) -> Parameter {
    // Check if parameter is optional (only is not already optional)
    if required {
        for (key, value) in &schema.extensions {
            if key == "nullable" {
                if let Some(nullable) = value.as_bool() {
                    required = !nullable;
                }
            }
        }
    }
    let description = schema.metadata.as_ref().and_then(|m| m.description.clone());
    Parameter {
        name,
        location: "query".to_owned(),
        description,
        required,
        deprecated: false,
        allow_empty_value: false,
        value: ParameterValue::Schema {
            style: None,
            explode: None,
            allow_reserved: false,
            schema,
            example: None,
            examples: None,
        },
        extensions: okapi::Map::default(),
    }
}

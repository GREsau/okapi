use crate::openapi3::{Components, Info, OpenApi, PathItem, Responses, Tag};
use crate::{Map, MapEntry};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct MergeError {
    pub msg: String,
}

impl Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl MergeError {
    fn new<S: AsRef<str>>(msg: S) -> Self {
        MergeError {
            msg: msg.as_ref().to_owned(),
        }
    }
}

impl OpenApi {
    /// Merge the given OpenAPI spec into the current one.
    pub fn merge_spec<S: Display>(mut self, path_prefix: &S, s2: &Self) -> Result<(), MergeError> {
        merge_specs(&mut self, path_prefix, s2)
    }
}

/// Marge the list of all specs together into on big OpenApi object.
pub fn marge_spec_list<S: Display>(spec_list: &[(S, OpenApi)]) -> Result<OpenApi, MergeError> {
    let mut openapi_docs = OpenApi::new();
    for (path_prefix, spec) in spec_list {
        merge_specs(&mut openapi_docs, path_prefix, spec)?;
    }
    Ok(openapi_docs)
}

/// Merge the given OpenAPI spec into the current one.
pub fn merge_specs<S: Display>(
    s1: &mut OpenApi,
    path_prefix: &S,
    s2: &OpenApi,
) -> Result<(), MergeError> {
    // Check if specs are same version
    if s1.openapi != s2.openapi {
        return Err(MergeError::new("OpenAPI specs version do not match."));
    }
    merge_spec_info(&mut s1.info, &s2.info)?;
    merge_vec(&mut s1.servers, &s2.servers);
    merge_paths(&mut s1.paths, path_prefix, &s2.paths)?;
    merge_components(&mut s1.components, &s2.components)?;
    // This is a `Vec<Map<String, _>` but just merge the `Vec` items together.
    // Do not merge the `Map` items together.
    merge_vec(&mut s1.security, &s2.security);
    merge_tags(&mut s1.tags, &s2.tags)?;
    // Replace the external_docs info as 1 block, so don't mix
    merge_option(&mut s1.external_docs, &s2.external_docs);
    merge_map(&mut s1.extensions, &s2.extensions, "extensions");
    Ok(())
}

pub fn merge_spec_info(s1: &mut Info, s2: &Info) -> Result<(), MergeError> {
    s1.title = merge_string(&s1.title, &s2.title);
    merge_opt_string(&mut s1.description, &s2.description);
    merge_opt_string(&mut s1.terms_of_service, &s2.terms_of_service);
    // Replace the contact info as 1 block, so don't mix
    merge_option(&mut s1.contact, &s2.contact);
    // Replace the license info as 1 block, so don't mix
    merge_option(&mut s1.license, &s2.license);
    s1.version = merge_string(&s1.version, &s2.version);
    merge_map(&mut s1.extensions, &s2.extensions, "extensions");
    Ok(())
}

/// Merge `Map<String, PathItem>`/`&Map<String, PathItem>`:
/// Merge together. If key already exists, use s1 version.
/// Use `path_prefix` in order to specify the mounting points for the routes.
pub fn merge_paths<S: Display>(
    s1: &mut Map<String, PathItem>,
    path_prefix: &S,
    s2: &Map<String, PathItem>,
) -> Result<(), MergeError> {
    // Add all s2 values
    // (if key does not already exists)
    for (key, value) in s2 {
        let new_key = if key.starts_with('/') {
            format!("{}{}", path_prefix, key)
        } else {
            log::error!(
                "All routes should have a leading '/' but non found in `{}`.",
                key
            );
            format!("{}/{}", path_prefix, key)
        };
        match s1.entry(new_key) {
            MapEntry::Occupied(mut entry) => {
                // Merge `PathItem` so get/post/put routes are getting merged
                let current_value = entry.get_mut();
                merge_path_item(current_value, value)?;
            }
            MapEntry::Vacant(entry) => {
                entry.insert(value.clone());
            }
        }
    }
    Ok(())
}

pub fn merge_path_item(s1: &mut PathItem, s2: &PathItem) -> Result<(), MergeError> {
    merge_opt_string(&mut s1.reference, &s2.reference);
    merge_opt_string(&mut s1.summary, &s2.summary);
    merge_opt_string(&mut s1.description, &s2.description);

    merge_option(&mut s1.get, &s2.get);
    merge_option(&mut s1.put, &s2.put);
    merge_option(&mut s1.post, &s2.post);
    merge_option(&mut s1.delete, &s2.delete);
    merge_option(&mut s1.options, &s2.options);
    merge_option(&mut s1.head, &s2.head);
    merge_option(&mut s1.patch, &s2.patch);
    merge_option(&mut s1.trace, &s2.trace);

    merge_option(&mut s1.servers, &s2.servers);
    merge_vec(&mut s1.parameters, &s2.parameters);
    merge_map(&mut s1.extensions, &s2.extensions, "extensions");
    Ok(())
}

pub fn merge_components(
    s1: &mut Option<Components>,
    s2: &Option<Components>,
) -> Result<(), MergeError> {
    if s1.is_none() {
        *s1 = s2.clone();
        Ok(())
    } else if s2.is_none() {
        // Use/keep s1
        Ok(())
    } else {
        if let Some(s1) = s1 {
            let s2 = s2.as_ref().unwrap();
            merge_map(&mut s1.schemas, &s2.schemas, "schemas");
            merge_map(&mut s1.responses, &s2.responses, "responses");
            merge_map(&mut s1.parameters, &s2.parameters, "parameters");
            merge_map(&mut s1.examples, &s2.examples, "examples");
            merge_map(&mut s1.request_bodies, &s2.request_bodies, "request_bodies");
            merge_map(&mut s1.headers, &s2.headers, "headers");
            merge_map(
                &mut s1.security_schemes,
                &s2.security_schemes,
                "security_schemes",
            );
            merge_map(&mut s1.links, &s2.links, "links");
            merge_map(&mut s1.callbacks, &s2.callbacks, "callbacks");
            merge_map(&mut s1.extensions, &s2.extensions, "extensions");
        }
        Ok(())
    }
}

pub fn merge_tags(s1: &mut Vec<Tag>, s2: &[Tag]) -> Result<Vec<Tag>, MergeError> {
    // Create a `Map` so we can easily merge tag names.
    let mut new_tags: Map<String, Tag> = Map::new();
    // Add all s1 tags
    for tag in s1 {
        match new_tags.entry(tag.name.clone()) {
            MapEntry::Occupied(mut entry) => {
                let current_value = entry.get_mut();
                merge_tag(current_value, tag)?;
            }
            MapEntry::Vacant(entry) => {
                entry.insert(tag.clone());
            }
        }
    }
    // Add all s2 tags
    for tag in s2 {
        match new_tags.entry(tag.name.clone()) {
            MapEntry::Occupied(mut entry) => {
                let current_value = entry.get_mut();
                merge_tag(current_value, tag)?;
            }
            MapEntry::Vacant(entry) => {
                entry.insert(tag.clone());
            }
        }
    }
    // Convert `Map` to `Vec`
    let mut new_tags_vec = Vec::new();

    // Remove/add in order
    // Clone all keys because that is faster then cloning all the tags.
    // `BTreeMap` does not implement `pop()` so can not use that.
    // Code below works both for `BTreeMap` as for `IndexMap`.
    let keys: Vec<String> = new_tags.keys().cloned().collect();
    for key in keys {
        if let Some(tag) = new_tags.remove(&key) {
            new_tags_vec.push(tag);
        } else {
            unreachable!("List sizes or same list do not match.");
        }
    }
    Ok(new_tags_vec)
}

pub fn merge_tag(s1: &mut Tag, s2: &Tag) -> Result<(), MergeError> {
    if s1.name != s2.name {
        return Err(MergeError::new("Tried to merge Tags with different names."));
    }
    merge_opt_string(&mut s1.description, &s2.description);
    merge_option(&mut s1.external_docs, &s2.external_docs);
    merge_map(&mut s1.extensions, &s2.extensions, "extensions");
    Ok(())
}

pub fn merge_responses(s1: &mut Responses, s2: &Responses) -> Result<(), MergeError> {
    merge_option(&mut s1.default, &s2.default);
    merge_map(&mut s1.responses, &s2.responses, "responses");
    merge_map(&mut s1.extensions, &s2.extensions, "extensions");
    Ok(())
}

/// Merge `String`/`&str`:
/// - If one is empty: Use other
/// - Otherwise: Use first value
pub fn merge_string(s1: &str, s2: &str) -> String {
    if s1.is_empty() {
        s2.to_owned()
    } else {
        s1.to_owned()
    }
}

/// Merge `Option<String>`/`&Option<String>`:
/// - If one is `None`: Use other
/// - If both are `Some`: Merge `String`
/// - Otherwise: Use first value
pub fn merge_opt_string(s1: &mut Option<String>, s2: &Option<String>) {
    if s1.is_none() {
        *s1 = s2.clone();
    } else if s1.is_some() && s2.is_some() {
        *s1 = Some(merge_string(s1.as_ref().unwrap(), s2.as_ref().unwrap()))
    }
}

/// Merge `Option<T>`/`&Option<T>`:
/// - If one is `None`: Use other
/// - Otherwise: Use first value
pub fn merge_option<T: Clone>(s1: &mut Option<T>, s2: &Option<T>) {
    if s1.is_none() {
        *s1 = s2.clone();
    }
}

/// Merge `Map<String, _>`/`&Map<String, _>`:
/// Merge together. If key already exists, use s1 version.
pub fn merge_map<T: Clone + PartialEq>(s1: &mut Map<String, T>, s2: &Map<String, T>, name: &str) {
    // Add all s2 values
    // (if key does not already exists)
    for (key, value) in s2 {
        if let Some(s1_value) = s1.get(key) {
            // Check if this is the same element
            if value != s1_value {
                log::warn!(
                    "Found conflicting {} keys while merging, \
                    they have the same name but different values: `{}`",
                    name,
                    key
                );
            }
        } else {
            s1.insert(key.clone(), value.clone());
        }
    }
}

/// Merge `Vec<_>`/`&Vec<_>`:
/// Append lists, `s1` first and `s2` after that.
pub fn merge_vec<T: Clone>(s1: &mut Vec<T>, s2: &[T]) {
    // Add all s2 values
    for value in s2 {
        s1.push(value.clone());
    }
}

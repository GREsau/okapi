#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub type Map<K, V> = schemars::Map<K, V>;
pub type MapEntry<'a, K, V> = schemars::MapEntry<'a, K, V>;

pub mod merge;
pub mod openapi3;

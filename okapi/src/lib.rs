#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub type Map<K, V> = schemars::Map<K, V>;

pub mod openapi3;

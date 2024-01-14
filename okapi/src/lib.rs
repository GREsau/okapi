#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub type Map<K, V> = schemars::Map<K, V>;
pub type MapEntry<'a, K, V> = schemars::MapEntry<'a, K, V>;

pub mod merge;
pub mod openapi3;

/// Re-export the current version of `Schemars` used by `Okapi`.
pub use schemars;

/// Macro to crate an `okapi::Map` with a number of key-value pairs in it.
///
/// # Examples
///
/// ```rust
/// use okapi::Map;
/// use okapi::map;
///
/// let my_map = map!{
///     "user:read".to_owned() => "Ability to read user data".to_owned(),
///     "user:write".to_owned() => "Ability to write user data".to_owned(),
/// };
///
/// let mut control = Map::new();
/// control.insert("user:read".to_owned(),"Ability to read user data".to_owned());
/// control.insert("user:write".to_owned(),"Ability to write user data".to_owned());
///
/// assert_eq!(my_map, control);
/// ```
#[macro_export]
macro_rules! map {
    ($($key:expr => $val:expr),* $(,)*) => ({
        #[allow(unused_mut)]
        let mut map = $crate::Map::new();
        $( map.insert($key, $val); )*
        map
    });
}

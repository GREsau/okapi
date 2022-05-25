pub extern crate uuid as uuid_crate;

use std::{fmt};
use std::str::FromStr;
use std::ops::Deref;

use rocket::form::FromFormField;
use rocket::request::{FromParam};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Uuid(uuid_crate::Uuid);

impl Uuid {
    pub fn new_v4() -> Self {
        Uuid(uuid_crate::Uuid::new_v4())
    }

    #[inline(always)]
    pub fn into_inner(self) -> uuid_crate::Uuid {
        self.0
    }
}

impl fmt::Display for Uuid {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> FromParam<'a> for Uuid {
    type Error = uuid_crate::Error;

    /// A value is successfully parsed if `param` is a properly formatted Uuid.
    /// Otherwise, a `ParseError` is returned.
    #[inline(always)]
    fn from_param(param: &'a str) -> Result<Uuid, Self::Error> {
        param.parse()
    }
}

impl rocket_okapi::JsonSchema for Uuid {
    fn schema_name() -> String {
        "uuid".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            ..Default::default()
        })
    }
}

impl FromFormField<'_> for Uuid {
    fn from_value(_field: rocket::form::ValueField<'_>) -> rocket::form::Result<'_, Self> {
        Ok(Uuid::new_v4())
    }
}

impl FromStr for Uuid {
    type Err = uuid_crate::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Uuid, Self::Err> {
        s.parse().map(Uuid)
    }
}

impl Deref for Uuid {
    type Target = uuid_crate::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<uuid_crate::Uuid> for Uuid {
    #[inline(always)]
    fn eq(&self, other: &uuid_crate::Uuid) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod test {
    use super::uuid_crate;
    use super::Uuid;
    use super::FromParam;
    use super::FromStr;

    #[test]
    fn test_from_str() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = Uuid::from_str(uuid_str).unwrap();
        assert_eq!(uuid_str, uuid_wrapper.to_string())
    }

    #[test]
    fn test_from_param() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = Uuid::from_param(uuid_str).unwrap();
        assert_eq!(uuid_str, uuid_wrapper.to_string())
    }

    #[test]
    fn test_into_inner() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = Uuid::from_param(uuid_str).unwrap();
        let real_uuid: uuid_crate::Uuid = uuid_str.parse().unwrap();
        let inner_uuid: uuid_crate::Uuid = uuid_wrapper.into_inner();
        assert_eq!(real_uuid, inner_uuid)
    }

    #[test]
    fn test_partial_eq() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
        let uuid_wrapper = Uuid::from_param(uuid_str).unwrap();
        let real_uuid: uuid_crate::Uuid = uuid_str.parse().unwrap();
        assert_eq!(uuid_wrapper, real_uuid)
    }

    #[test]
    #[should_panic(expected = "InvalidLength")]
    fn test_from_param_invalid() {
        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2p";
        Uuid::from_param(uuid_str).unwrap();
    }
}

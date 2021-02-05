use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
/// Defined by [IETF BCP 47](https://tools.ietf.org/html/bcp47).
pub struct Lang(String);

impl_characters_yaserialize_yadeserialize!(Lang, "Lang");

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

impl From<&str> for Lang {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<String> for Lang {
    fn from(s: String) -> Self {
        Self(s)
    }
}

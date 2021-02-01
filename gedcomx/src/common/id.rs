use serde::{Deserialize, Serialize};
use std::fmt;

/// A local, context-specific id for the data.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Id(pub(crate) String);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

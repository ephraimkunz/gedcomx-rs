use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{EnumAsString, Uri};

/// Levels of confidence.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum ConfidenceLevel {
    /// The contributor has a high degree of confidence that the assertion is
    /// true.
    High,
    /// The contributor has a medium degree of confidence that the assertion is
    /// true.
    Medium,
    /// The contributor has a low degree of confidence that the assertion is
    /// true.
    Low,
    Custom(Uri),
}

impl_enumasstring_yaserialize_yadeserialize!(ConfidenceLevel, "ConfidenceLevel");

impl From<EnumAsString> for ConfidenceLevel {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/High" => Self::High,
            "http://gedcomx.org/Medium" => Self::Medium,
            "http://gedcomx.org/Low" => Self::Low,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for ConfidenceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::High => write!(f, "http://gedcomx.org/High"),
            Self::Medium => write!(f, "http://gedcomx.org/Medium"),
            Self::Low => write!(f, "http://gedcomx.org/Low"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

impl Default for ConfidenceLevel {
    fn default() -> Self {
        Self::Custom(Uri::default())
    }
}

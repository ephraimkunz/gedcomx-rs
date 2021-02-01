use crate::{EnumAsString, Uri};
use serde::{Deserialize, Serialize};
use std::fmt;

// I think this will need custom JSON serialization / deserialization. Needs to be a map of typee -> [uri].
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Identifier {
    pub uri: Uri,
    pub typee: Option<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum IdentifierType {
    Primary,
    Authority,
    Deprecated,
    Custom(Uri),
}

impl From<EnumAsString> for IdentifierType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Primary" => Self::Primary,
            "http://gedcomx.org/Authority" => Self::Authority,
            "http://gedcomx.org/Deprecated" => Self::Deprecated,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for IdentifierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Primary => write!(f, "http://gedcomx.org/Primary"),
            Self::Authority => write!(f, "http://gedcomx.org/Authority"),
            Self::Deprecated => write!(f, "http://gedcomx.org/Deprecated"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

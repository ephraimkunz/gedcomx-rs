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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn roundtrip_to_string() {
        let variant = ConfidenceLevel::High;
        let s = variant.to_string();
        let roundtripped: ConfidenceLevel = EnumAsString::from(s).into();
        assert_eq!(variant, roundtripped);
    }

    #[test]
    fn roundtrip_to_string_custom() {
        let variant = ConfidenceLevel::Custom("custom uri".into());
        let s = variant.to_string();
        let roundtripped: ConfidenceLevel = EnumAsString::from(s).into();
        assert_eq!(variant, roundtripped);
    }

    #[test]
    fn deserialize() {
        let xml = "<ConfidenceLevel>http://gedcomx.org/High</ConfidenceLevel>";
        let cl: ConfidenceLevel = yaserde::de::from_str(xml).unwrap();
        assert_eq!(cl, ConfidenceLevel::High)
    }

    #[test]
    fn deserialize_custom() {
        let xml = "<ConfidenceLevel>this is a test</ConfidenceLevel>";
        let cl: ConfidenceLevel = yaserde::de::from_str(xml).unwrap();
        assert_eq!(cl, ConfidenceLevel::Custom("this is a test".into()))
    }

    #[test]
    fn serialize() {
        let mut config = yaserde::ser::Config::default();
        config.write_document_declaration = false;
        let actual = yaserde::ser::to_string_with_config(&ConfidenceLevel::High, &config).unwrap();
        let expected = "http://gedcomx.org/High";

        assert_eq!(actual, expected)
    }

    #[test]
    fn serialize_custom() {
        let mut config = yaserde::ser::Config::default();
        config.write_document_declaration = false;
        let actual = yaserde::ser::to_string_with_config(
            &ConfidenceLevel::Custom("this is a test".into()),
            &config,
        )
        .unwrap();
        let expected = "this is a test";

        assert_eq!(actual, expected)
    }
}

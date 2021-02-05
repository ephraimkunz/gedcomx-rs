use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A concluded genealogical date.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Date {
    /// The original value of the date as supplied by the contributor.
    pub original: Option<String>,

    /// The standardized formal value of the date, formatted according to the GEDCOM X Date Format specification.
    // TODO: I think I should be using a different type for this one.
    pub formal: Option<String>,
}

impl Date {
    pub fn new<I: Into<String>>(original: Option<I>, formal: Option<I>) -> Self {
        Self {
            original: original.map(std::convert::Into::into),
            formal: formal.map(std::convert::Into::into),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "original" : "the original text"
          }"#;

        let date: Date = serde_json::from_str(json).unwrap();

        assert_eq!(
            date,
            Date {
                original: Some("the original text".to_string()),
                formal: None // Replace with formal date once we implement that.
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{}"#;

        let date: Date = serde_json::from_str(json).unwrap();

        assert_eq!(
            date,
            Date {
                original: None,
                formal: None
            }
        )
    }

    #[test]
    fn json_serialize() {
        let date = Date {
            original: Some("the original text".to_string()),
            formal: None, // Replace with formal date once we implement that.
        };

        let json = serde_json::to_string(&date).unwrap();

        assert_eq!(json, r#"{"original":"the original text"}"#)
    }

    #[test]
    fn json_serialize_optional_fields() {
        let date = Date {
            original: None,
            formal: None,
        };

        let json = serde_json::to_string(&date).unwrap();

        assert_eq!(json, r#"{}"#)
    }
}

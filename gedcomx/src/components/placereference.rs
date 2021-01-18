use crate::Uri;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct PlaceReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "description")]
    pub description_ref: Option<Uri>,
}

impl PlaceReference {
    pub fn new() -> Self {
        Self {
            original: None,
            description_ref: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "original" : "the original text",
            "description" : "http://identifier/of/place-description/being/referenced"          
          }"#;

        let place_ref: PlaceReference = serde_json::from_str(json).unwrap();

        assert_eq!(
            place_ref,
            PlaceReference {
                original: Some("the original text".to_string()),
                description_ref: Some(
                    "http://identifier/of/place-description/being/referenced".into()
                )
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{}"#;

        let place_ref: PlaceReference = serde_json::from_str(json).unwrap();

        assert_eq!(place_ref, PlaceReference::new())
    }

    #[test]
    fn json_serialize() {
        let place_ref = PlaceReference {
            original: Some("the original text".to_string()),
            description_ref: Some("http://identifier/of/place-description/being/referenced".into()),
        };

        let json = serde_json::to_string(&place_ref).unwrap();

        assert_eq!(
            json,
            r#"{"original":"the original text","description":"http://identifier/of/place-description/being/referenced"}"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let place_ref = PlaceReference::new();

        let json = serde_json::to_string(&place_ref).unwrap();

        assert_eq!(json, r#"{}"#)
    }
}

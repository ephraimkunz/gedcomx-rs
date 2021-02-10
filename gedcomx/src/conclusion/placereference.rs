use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{PlaceDescription, Result, Uri};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct PlaceReference {
    pub original: Option<String>,

    #[serde(rename = "description")]
    pub description_ref: Option<Uri>,
}

impl PlaceReference {
    pub fn new(original: Option<String>, description_ref: Option<Uri>) -> Self {
        Self {
            original,
            description_ref,
        }
    }

    pub fn builder() -> PlaceReferenceBuilder {
        PlaceReferenceBuilder::new()
    }
}

pub struct PlaceReferenceBuilder(PlaceReference);

impl PlaceReferenceBuilder {
    pub(crate) fn new() -> Self {
        Self(PlaceReference::default())
    }

    pub fn description_ref(&mut self, description: &PlaceDescription) -> Result<&mut Self> {
        self.0.description_ref = Some(description.try_into()?);
        Ok(self)
    }

    pub fn original<I: Into<String>>(&mut self, original: I) -> &mut Self {
        self.0.original = Some(original.into());
        self
    }

    pub fn build(&self) -> PlaceReference {
        PlaceReference::new(self.0.original.clone(), self.0.description_ref.clone())
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

        assert_eq!(place_ref, PlaceReference::default())
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
        let place_ref = PlaceReference::default();

        let json = serde_json::to_string(&place_ref).unwrap();

        assert_eq!(json, r#"{}"#)
    }
}

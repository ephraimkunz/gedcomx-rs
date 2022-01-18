use std::convert::TryInto;

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{PlaceDescription, Result, Uri};

/// A reference to a description of a place.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct PlaceReference {
    /// The original place name text as supplied by the contributor.
    #[yaserde(prefix = "gx")]
    pub original: Option<String>,

    /// A reference to a description of this place.
    ///
    /// MUST resolve to a PlaceDescription.
    #[yaserde(attribute, rename = "description")]
    #[serde(rename = "description")]
    pub description_ref: Option<Uri>,
}

impl PlaceReference {
    pub fn new<I: Into<String>>(original: Option<I>, description_ref: Option<Uri>) -> Self {
        Self {
            original: original.map(std::convert::Into::into),
            description_ref,
        }
    }

    pub fn builder() -> PlaceReferenceBuilder {
        PlaceReferenceBuilder::new()
    }
}

impl Arbitrary for PlaceReference {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut place_reference = Self::builder()
            .original(crate::arbitrary_trimmed(g))
            .build();

        place_reference.description_ref = Some(Uri::arbitrary(g));

        place_reference
    }
}

pub struct PlaceReferenceBuilder(PlaceReference);

impl PlaceReferenceBuilder {
    pub(crate) fn new() -> Self {
        Self(PlaceReference::default())
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`Uri`](crate::Uri) fails.
    /// This happens if `description` has no `id` set.
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
    use pretty_assertions::assert_eq;

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
    fn xml_deserialize() {
        let xml = r#"<PlaceReference description="http://identifier/of/place/description/being/referenced">
        <original>the original text</original>
      </PlaceReference>"#;

        let place_ref: PlaceReference = yaserde::de::from_str(xml).unwrap();

        assert_eq!(
            place_ref,
            PlaceReference {
                original: Some("the original text".to_string()),
                description_ref: Some(
                    "http://identifier/of/place/description/being/referenced".into()
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
            description_ref: Some("http://identifier/of/place/description/being/referenced".into()),
        };

        let json = serde_json::to_string(&place_ref).unwrap();

        assert_eq!(
            json,
            r#"{"original":"the original text","description":"http://identifier/of/place/description/being/referenced"}"#
        )
    }

    #[test]
    fn xml_serialize() {
        let place_ref = PlaceReference {
            original: Some("the original text".to_string()),
            description_ref: Some("http://identifier/of/place/description/being/referenced".into()),
        };

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };

        let xml = yaserde::ser::to_string_with_config(&place_ref, &config).unwrap();

        assert_eq!(
            xml,
            r#"<PlaceReference xmlns="http://gedcomx.org/v1/" description="http://identifier/of/place/description/being/referenced"><original>the original text</original></PlaceReference>"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let place_ref = PlaceReference::default();

        let json = serde_json::to_string(&place_ref).unwrap();

        assert_eq!(json, r#"{}"#)
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: PlaceReference) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: PlaceReference = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: PlaceReference) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: PlaceReference = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

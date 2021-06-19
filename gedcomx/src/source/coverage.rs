use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Date, PlaceReference};

/// The coverage of a resource.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct Coverage {
    /// The spatial (i.e., geographic) coverage.
    #[yaserde(prefix = "gx")]
    pub spatial: Option<PlaceReference>,

    /// The temporal coverage.
    #[yaserde(prefix = "gx")]
    pub temporal: Option<Date>,
}

impl Coverage {
    pub fn new(spatial: Option<PlaceReference>, temporal: Option<Date>) -> Self {
        Self { spatial, temporal }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "spatial" : { "original": "Place reference"},
            "temporal" : { "original": "Original date" }
          }"#;

        let coverage: Coverage = serde_json::from_str(json).unwrap();

        assert_eq!(
            coverage,
            Coverage::new(
                Some(
                    PlaceReference::builder()
                        .original("Place reference")
                        .build()
                ),
                Some(Date::new(Some("Original date"), None))
            )
        )
    }

    #[test]
    fn xml_deserialize() {
        let xml = r#"  <Coverage>
        <spatial>
          <original>Place reference</original>
        </spatial>
        <temporal>
          <original>Original date</original>
        </temporal>
    
      </Coverage>"#;

        let coverage: Coverage = yaserde::de::from_str(xml).unwrap();

        assert_eq!(
            coverage,
            Coverage::new(
                Some(
                    PlaceReference::builder()
                        .original("Place reference")
                        .build()
                ),
                Some(Date::new(Some("Original date"), None))
            )
        )
    }

    #[test]
    fn json_serialize() {
        let coverage = Coverage::new(
            Some(
                PlaceReference::builder()
                    .original("Place reference")
                    .build(),
            ),
            Some(Date::new(Some("Original date"), None)),
        );
        let json = serde_json::to_string(&coverage).unwrap();

        assert_eq!(
            json,
            r#"{"spatial":{"original":"Place reference"},"temporal":{"original":"Original date"}}"#
        )
    }

    #[test]
    fn xml_serialize() {
        let coverage = Coverage::new(
            Some(
                PlaceReference::builder()
                    .original("Place reference")
                    .build(),
            ),
            Some(Date::new(Some("Original date"), None)),
        );

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };

        let xml = yaserde::ser::to_string_with_config(&coverage, &config).unwrap();

        let expected_xml = r#"<Coverage xmlns="http://gedcomx.org/v1/"><spatial><original>Place reference</original></spatial><temporal><original>Original date</original></temporal></Coverage>"#;

        assert_eq!(xml, expected_xml)
    }
}

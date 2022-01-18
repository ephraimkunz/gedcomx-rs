use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::GedcomxDate;

/// A concluded genealogical date.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct Date {
    /// The original value of the date as supplied by the contributor.
    #[yaserde(prefix = "gx")]
    pub original: Option<String>,

    /// The standardized formal value of the date, formatted according to the
    /// GEDCOM X Date Format specification.
    #[yaserde(prefix = "gx")]
    pub formal: Option<GedcomxDate>,
}

impl Date {
    pub fn new<I: Into<String>>(original: Option<I>, formal: Option<GedcomxDate>) -> Self {
        Self {
            original: original.map(std::convert::Into::into),
            formal,
        }
    }
}

impl Arbitrary for Date {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(
            Some(crate::arbitrary_trimmed(g)),
            Some(GedcomxDate::arbitrary(g)),
        )
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
            "formal" : "+0987-01-25T23:59Z"
          }"#;

        let date: Date = serde_json::from_str(json).unwrap();

        assert_eq!(
            date,
            Date {
                original: Some("the original text".to_string()),
                formal: Some("+0987-01-25T23:59Z".parse().unwrap())
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
            formal: Some("+0987-01-25T23:59Z".parse().unwrap()),
        };

        let json = serde_json::to_string(&date).unwrap();

        assert_eq!(
            json,
            r#"{"original":"the original text","formal":"+0987-01-25T23:59Z"}"#
        )
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

    #[test]
    fn xml_deserialize() {
        let xml = r#"
        <date>
            <original>the original text</original>
            <formal>+0987-01-25T23:59Z</formal>
        </date>"#;

        let date: Date = yaserde::de::from_str(xml).unwrap();

        assert_eq!(
            date,
            Date {
                original: Some("the original text".to_string()),
                formal: Some("+0987-01-25T23:59Z".parse().unwrap())
            }
        )
    }

    #[test]
    fn xml_deserialize_optional_fields() {
        let xml = r#"<date />"#;

        let date: Date = yaserde::de::from_str(xml).unwrap();

        assert_eq!(
            date,
            Date {
                original: None,
                formal: None
            }
        )
    }

    #[test]
    fn xml_serialize() {
        let date = Date {
            original: Some("the original text".to_string()),
            formal: Some("+0987-01-25T23:59Z".parse().unwrap()),
        };

        let mut config = yaserde::ser::Config::default();
        config.write_document_declaration = false;
        let xml = yaserde::ser::to_string_with_config(&date, &config).unwrap();

        assert_eq!(
            xml,
            r#"<Date xmlns="http://gedcomx.org/v1/"><original>the original text</original><formal>+0987-01-25T23:59Z</formal></Date>"#
        )
    }

    #[test]
    fn xml_serialize_optional_fields() {
        let date = Date {
            original: None,
            formal: None,
        };

        let mut config = yaserde::ser::Config::default();
        config.write_document_declaration = false;
        let xml = yaserde::ser::to_string_with_config(&date, &config).unwrap();

        assert_eq!(xml, r#"<Date xmlns="http://gedcomx.org/v1/" />"#)
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: Date) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: Date = serde_json::from_str(&json).unwrap();
        assert_eq!(input, from_json);
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: Date) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: Date = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

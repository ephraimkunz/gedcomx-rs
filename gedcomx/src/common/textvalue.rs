use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::Lang;

/// An element representing a text value that may be in a specific language.
#[skip_serializing_none]
#[derive(
    Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default, Eq,
)]
#[non_exhaustive]
pub struct TextValue {
    /// The locale identifier for the value of the text.
    #[yaserde(attribute, prefix = "xml")]
    pub lang: Option<Lang>,

    /// The text value.
    #[yaserde(text)]
    pub value: String,
}

impl TextValue {
    pub fn new<I, J>(value: I, lang: Option<J>) -> Self
    where
        I: Into<String>,
        J: Into<Lang>,
    {
        Self {
            value: value.into(),
            lang: lang.map(std::convert::Into::into),
        }
    }
}

impl From<&str> for TextValue {
    fn from(s: &str) -> Self {
        Self {
            value: s.into(),
            ..Self::default()
        }
    }
}

impl Arbitrary for TextValue {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(crate::arbitrary_trimmed(g), Some(Lang::arbitrary(g)))
    }
}

#[cfg(test)]
mod test {
    use yaserde::ser::Config;

    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "lang" : "en",
            "value" : "text of the value"
        }"#;

        let text_value: TextValue = serde_json::from_str(json).unwrap();
        assert_eq!(
            text_value,
            TextValue {
                lang: Some("en".into()),
                value: "text of the value".to_string(),
            }
        );
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "value" : "text of the value"
        }"#;

        let text_value: TextValue = serde_json::from_str(json).unwrap();
        assert_eq!(text_value, TextValue::from("text of the value"));
    }

    #[test]
    fn json_serialize() {
        let text_value = TextValue {
            lang: Some("en".into()),
            value: "text of the value".to_string(),
        };

        let json = serde_json::to_string(&text_value).unwrap();

        assert_eq!(json, r#"{"lang":"en","value":"text of the value"}"#);
    }

    #[test]
    fn json_serialize_optional_fields() {
        let text_value = TextValue::from("text of the value");

        let json = serde_json::to_string(&text_value).unwrap();

        assert_eq!(json, r#"{"value":"text of the value"}"#);
    }

    #[test]
    fn xml_serialize() {
        let textvalue = TextValue::new("...textual value...", Some("en"));

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };

        let xml = yaserde::ser::to_string_with_config(&textvalue, &config).unwrap();

        let expected = r##"<TextValue xml:lang="en">...textual value...</TextValue>"##;

        assert_eq!(xml, expected);
    }

    #[test]
    fn xml_deserialize() {
        let xml = r##"<TextValue xml:lang="en">...textual value...</TextValue>"##;

        let textvalue: TextValue = yaserde::de::from_str(xml).unwrap();
        let expected = TextValue::new("...textual value...", Some("en"));
        assert_eq!(textvalue, expected);
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: TextValue) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: TextValue = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: TextValue) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: TextValue = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

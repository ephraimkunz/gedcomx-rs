use crate::Lang;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

/// An element representing a text value that may be in a specific language.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct TextValue {
    /// The locale identifier for the value of the text.
    pub lang: Option<Lang>,

    /// The text value.
    pub value: String,
}

impl TextValue {
    pub fn new<I: Into<String>>(value: I, lang: Option<Lang>) -> Self {
        Self {
            value: value.into(),
            lang,
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

#[cfg(test)]
mod test {
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
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "value" : "text of the value"
        }"#;

        let text_value: TextValue = serde_json::from_str(json).unwrap();
        assert_eq!(text_value, TextValue::new("text of the value", None))
    }

    #[test]
    fn json_serialize() {
        let text_value = TextValue {
            lang: Some("en".into()),
            value: "text of the value".to_string(),
        };

        let json = serde_json::to_string(&text_value).unwrap();

        assert_eq!(json, r#"{"lang":"en","value":"text of the value"}"#)
    }

    #[test]
    fn json_serialize_optional_fields() {
        let text_value = TextValue::new("text of the value", None);

        let json = serde_json::to_string(&text_value).unwrap();

        assert_eq!(json, r#"{"value":"text of the value"}"#)
    }
}

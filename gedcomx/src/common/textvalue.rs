use crate::Lang;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct TextValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    pub value: String,
}

impl TextValue {
    pub fn new(value: String, lang: Option<Lang>) -> Self {
        Self { value, lang }
    }

    pub fn builder<I: Into<String>>(value: I) -> TextValueBuilder {
        TextValueBuilder::new(value)
    }
}

pub struct TextValueBuilder(TextValue);

impl TextValueBuilder {
    pub(crate) fn new<I: Into<String>>(value: I) -> Self {
        Self(TextValue {
            value: value.into(),
            ..TextValue::default()
        })
    }

    pub fn build(&self) -> TextValue {
        TextValue::new(self.0.value.clone(), self.0.lang.clone())
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
                lang: Some("en".to_string()),
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
        assert_eq!(text_value, TextValue::builder("text of the value").build())
    }

    #[test]
    fn json_serialize() {
        let text_value = TextValue {
            lang: Some("en".to_string()),
            value: "text of the value".to_string(),
        };

        let json = serde_json::to_string(&text_value).unwrap();

        assert_eq!(json, r#"{"lang":"en","value":"text of the value"}"#)
    }

    #[test]
    fn json_serialize_optional_fields() {
        let text_value = TextValue::builder("text of the value").build();

        let json = serde_json::to_string(&text_value).unwrap();

        assert_eq!(json, r#"{"value":"text of the value"}"#)
    }
}

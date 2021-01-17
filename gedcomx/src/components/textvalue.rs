use crate::components::Lang;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct TextValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    pub value: String,
}

impl TextValue {
    pub fn new(value: String) -> Self {
        Self { value, lang: None }
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
        assert_eq!(text_value, TextValue::new("text of the value".to_string()))
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
        let text_value = TextValue::new("text of the value".to_string());

        let json = serde_json::to_string(&text_value).unwrap();

        assert_eq!(json, r#"{"value":"text of the value"}"#)
    }
}

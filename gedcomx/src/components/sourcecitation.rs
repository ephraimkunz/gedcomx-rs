use crate::components::Lang;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct SourceCitation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,
    pub value: String,
}

impl SourceCitation {
    pub fn new(value: String) -> Self {
        Self { value, lang: None }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "lang" : "en",
            "value" : "a rendering of the full citation as a string"
        }"#;

        let source_citation: SourceCitation = serde_json::from_str(json).unwrap();
        assert_eq!(
            source_citation,
            SourceCitation {
                lang: Some("en".to_string()),
                value: "a rendering of the full citation as a string".to_string(),
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "value" : "a rendering of the full citation as a string"
        }"#;

        let source_citation: SourceCitation = serde_json::from_str(json).unwrap();
        assert_eq!(
            source_citation,
            SourceCitation::new("a rendering of the full citation as a string".to_string())
        )
    }

    #[test]
    fn json_serialize() {
        let source_citation = SourceCitation {
            lang: Some("en".to_string()),
            value: "a rendering of the full citation as a string".to_string(),
        };

        let json = serde_json::to_string(&source_citation).unwrap();

        assert_eq!(
            json,
            r#"{"lang":"en","value":"a rendering of the full citation as a string"}"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let source_citation =
            SourceCitation::new("a rendering of the full citation as a string".to_string());

        let json = serde_json::to_string(&source_citation).unwrap();

        assert_eq!(
            json,
            r#"{"value":"a rendering of the full citation as a string"}"#
        )
    }
}

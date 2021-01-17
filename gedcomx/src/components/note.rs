use crate::components::{Attribution, Lang};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct Note {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
}

impl Note {
    pub fn new(text: String) -> Self {
        Note {
            text,
            lang: None,
            subject: None,
            attribution: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::TestData;

    #[test]
    fn json_deserialize() {
        let data = TestData::new();

        let json = r#"{
            "lang" : "en",
            "subject" : "TestSubject",
            "text" : "This is a note",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            }        
        }"#;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(
            note,
            Note {
                lang: Some("en".to_string()),
                subject: Some("TestSubject".to_string()),
                text: "This is a note".to_string(),
                attribution: data.attribution(),
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "text" : "This is a note"      
        }"#;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note, Note::new("This is a note".to_string()))
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let note = Note {
            lang: Some("en".to_string()),
            subject: Some("TestSubject".to_string()),
            text: "This is a note".to_string(),
            attribution: data.attribution(),
        };

        let json = serde_json::to_string(&note).unwrap();

        assert_eq!(
            json,
            r#"{"lang":"en","subject":"TestSubject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}"#
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let note = Note::new("This is a note".to_string());

        let json = serde_json::to_string(&note).unwrap();

        assert_eq!(json, r#"{"text":"This is a note"}"#);
    }
}

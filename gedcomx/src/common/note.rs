use crate::{Attribution, Lang};
use serde::{Deserialize, Serialize};

/// A note that was contributed from genealogical research.
///
/// Notes are not intended to contain genealogical conclusions. Notes are only associated with a single genealogical resource.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Note {
    /// The locale identifier for the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,

    /// A subject or title for the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// The text of the note.
    pub text: String,

    /// The attribution of this note. If not provided, the attribution of the containing resource of the note is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
}

impl Note {
    pub fn new(text: String) -> Self {
        Self {
            text,
            lang: None,
            subject: None,
            attribution: None,
        }
    }
}

// TODO: Builder for this?

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestData;

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
                attribution: Some(data.attribution),
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
            attribution: Some(data.attribution),
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

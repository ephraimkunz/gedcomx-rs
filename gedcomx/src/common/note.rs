use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Attribution, Lang};

/// A note that was contributed from genealogical research.
///
/// Notes are not intended to contain genealogical conclusions. Notes are only
/// associated with a single genealogical resource.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct Note {
    /// The locale identifier for the note.
    #[yaserde(attribute, prefix = "xml")]
    pub lang: Option<Lang>,

    /// A subject or title for the note.
    #[yaserde(prefix = "gx")]
    pub subject: Option<String>,

    /// The text of the note.
    #[yaserde(prefix = "gx")]
    pub text: String,

    /// The attribution of this note. If not provided, the attribution of the
    /// containing resource of the note is assumed.
    #[yaserde(prefix = "gx")]
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
                lang: Some("en".into()),
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
            lang: Some("en".into()),
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

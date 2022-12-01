use quickcheck::{Arbitrary, Gen};
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
    rename = "note",
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
    pub fn new(
        lang: Option<Lang>,
        subject: Option<String>,
        text: String,
        attribution: Option<Attribution>,
    ) -> Self {
        Self {
            lang,
            subject,
            text,
            attribution,
        }
    }

    pub fn builder<I: Into<String>>(text: I) -> NoteBuilder {
        NoteBuilder::new(text)
    }
}

impl Arbitrary for Note {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::builder(crate::arbitrary_trimmed(g))
            .lang(Lang::arbitrary(g))
            .subject(crate::arbitrary_trimmed(g))
            .attribution(Attribution::arbitrary(g))
            .build()
    }
}

pub struct NoteBuilder(Note);

impl NoteBuilder {
    pub(crate) fn new<I: Into<String>>(text: I) -> Self {
        Self(Note {
            text: text.into(),
            ..Note::default()
        })
    }

    pub fn lang<I: Into<Lang>>(&mut self, lang: I) -> &mut Self {
        self.0.lang = Some(lang.into());
        self
    }

    pub fn subject<I: Into<String>>(&mut self, subject: I) -> &mut Self {
        self.0.subject = Some(subject.into());
        self
    }

    pub fn attribution(&mut self, attribution: Attribution) -> &mut Self {
        self.0.attribution = Some(attribution);
        self
    }

    pub fn build(&self) -> Note {
        Note::new(
            self.0.lang.clone(),
            self.0.subject.clone(),
            self.0.text.clone(),
            self.0.attribution.clone(),
        )
    }
}

#[cfg(test)]
mod test {
    use yaserde::ser::Config;

    use super::*;
    use crate::TestData;

    #[test]
    fn builder() {
        let expected = Note {
            lang: Some("en".into()),
            subject: Some("subject".to_string()),
            text: "text".to_string(),
            attribution: Some(Attribution::default()),
        };

        let actual = Note::builder("text")
            .lang("en")
            .subject("subject")
            .attribution(Attribution::default())
            .build();

        assert_eq!(actual, expected);
    }

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
        );
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "text" : "This is a note"      
        }"#;

        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note, Note::builder("This is a note").build());
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
        let note = Note::builder("This is a note").build();

        let json = serde_json::to_string(&note).unwrap();

        assert_eq!(json, r#"{"text":"This is a note"}"#);
    }

    #[test]
    fn xml_serialize() {
        let note = Note::builder("...text of the note...")
            .lang("en")
            .subject("...subject or title...")
            .attribution(Attribution::default())
            .build();

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };
        let xml = yaserde::ser::to_string_with_config(&note, &config).unwrap();
        let expected = r##"<note xmlns="http://gedcomx.org/v1/" xml:lang="en"><subject>...subject or title...</subject><text>...text of the note...</text><attribution /></note>"##;

        assert_eq!(xml, expected);
    }

    #[test]
    fn xml_deserialize() {
        let xml = r##"<note xml:lang="en">
        <subject>...subject or title...</subject>
        <text>...text of the note...</text>
        <attribution>
        </attribution>    
        </note>"##;

        let note: Note = yaserde::de::from_str(xml).unwrap();
        let expected = Note::builder("...text of the note...")
            .lang("en")
            .subject("...subject or title...")
            .attribution(Attribution::default())
            .build();
        assert_eq!(note, expected);
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: Note) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: Note = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: Note) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: Note = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

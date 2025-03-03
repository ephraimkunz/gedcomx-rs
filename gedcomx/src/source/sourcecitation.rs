use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::Lang;

/// A container for the metadata necessary for an agent to identify a source(s).
#[skip_serializing_none]
#[derive(
    Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default, Eq,
)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct SourceCitation {
    /// The locale identifier for the bibliographic metadata.
    #[yaserde(attribute, prefix = "xml")]
    pub lang: Option<Lang>,

    /// The bibliographic metadata rendered as a full citation.
    ///
    ///  This string is plain text, but MAY include an xhtml cite element. If
    /// the value includes a cite element, the text-level semantics defined for
    /// cite MUST apply—i.e., the element MUST represent the title of a work.
    #[yaserde(prefix = "gx")]
    pub value: String,
}

impl SourceCitation {
    pub fn new<I: Into<String>>(value: I, lang: Option<Lang>) -> Self {
        Self {
            lang,
            value: value.into(),
        }
    }
}

impl Arbitrary for SourceCitation {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(crate::arbitrary_trimmed(g), Some(Lang::arbitrary(g)))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::super::SourceCitation;

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
                lang: Some("en".into()),
                value: "a rendering of the full citation as a string".to_string(),
            }
        );
    }

    #[test]
    fn xml_deserialize() {
        let xml = r#"<SourceCitation xmlns="http://gedcomx.org/v1/" xml:lang="en">
        <value>a rendering of the full citation as a string</value>    
      </SourceCitation>"#;

        let source_citation: SourceCitation = yaserde::de::from_str(xml).unwrap();
        assert_eq!(
            source_citation,
            SourceCitation {
                lang: Some("en".into()),
                value: "a rendering of the full citation as a string".to_string(),
            }
        );
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "value" : "a rendering of the full citation as a string"
        }"#;

        let source_citation: SourceCitation = serde_json::from_str(json).unwrap();
        assert_eq!(
            source_citation,
            SourceCitation::new(
                "a rendering of the full citation as a string".to_string(),
                None
            )
        );
    }

    #[test]
    fn json_serialize() {
        let source_citation = SourceCitation {
            lang: Some("en".into()),
            value: "a rendering of the full citation as a string".to_string(),
        };

        let json = serde_json::to_string(&source_citation).unwrap();

        assert_eq!(
            json,
            r#"{"lang":"en","value":"a rendering of the full citation as a string"}"#
        );
    }

    #[test]
    fn xml_serialize() {
        let source_citation = SourceCitation {
            lang: Some("en".into()),
            value: "a rendering of the full citation as a string".to_string(),
        };

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };

        let xml = yaserde::ser::to_string_with_config(&source_citation, &config).unwrap();

        assert_eq!(
            xml,
            r#"<SourceCitation xmlns="http://gedcomx.org/v1/" xml:lang="en"><value>a rendering of the full citation as a string</value></SourceCitation>"#
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let source_citation = SourceCitation::new(
            "a rendering of the full citation as a string".to_string(),
            None,
        );

        let json = serde_json::to_string(&source_citation).unwrap();

        assert_eq!(
            json,
            r#"{"value":"a rendering of the full citation as a string"}"#
        );
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: SourceCitation) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: SourceCitation = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: SourceCitation) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: SourceCitation = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

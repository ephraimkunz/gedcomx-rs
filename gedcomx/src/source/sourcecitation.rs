use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::Lang;

/// A container for the metadata necessary for an agent to identify a source(s).
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct SourceCitation {
    /// The locale identifier for the bibliographic metadata.
    pub lang: Option<Lang>,

    /// The bibliographic metadata rendered as a full citation.
    ///
    ///  This string is plain text, but MAY include an xhtml cite element. If
    /// the value includes a cite element, the text-level semantics defined for
    /// cite MUST apply—i.e., the element MUST represent the title of a work.
    pub value: String,
}

impl SourceCitation {
    pub fn new(value: String, lang: Option<Lang>) -> Self {
        Self { value, lang }
    }

    pub fn builder<I: Into<String>>(value: I) -> SourceCitationBuilder {
        SourceCitationBuilder::new(value)
    }
}

pub struct SourceCitationBuilder(SourceCitation);

impl SourceCitationBuilder {
    pub(crate) fn new<I: Into<String>>(value: I) -> Self {
        Self(SourceCitation {
            value: value.into(),
            ..SourceCitation::default()
        })
    }

    pub fn build(&self) -> SourceCitation {
        SourceCitation::new(self.0.value.clone(), self.0.lang.clone())
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
                lang: Some("en".into()),
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
            SourceCitation::new(
                "a rendering of the full citation as a string".to_string(),
                None
            )
        )
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
        )
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
        )
    }
}

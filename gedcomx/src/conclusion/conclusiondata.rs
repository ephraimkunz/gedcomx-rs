use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Attribution, EnumAsString, Id, Lang, Note, ResourceReference, SourceReference, Uri};

/// The abstract concept for a basic genealogical data item.
///
/// In formal discussions of the genealogical research process, the term
/// "conclusion" usually has a more specific meaning and is used to refer to an
/// "accepted" hypothesis in accordance with the Genealogical Proof Standard.
/// The name of the `ConclusionData` type is not meant to be associated with the
/// definition of the term "conclusion" as it is described in the genealogical
/// research process. Rather, the name refers to the notion that any information
/// that is interpreted from an "original" is in some way a "conclusion"—even if
/// the interpreter was diligent in representing the information verbatim as it
/// was found in the original.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct ConclusionData {
    /// An identifier for the conclusion data. The id is to be used as a "fragment identifier" as defined by [RFC 3986, Section 3.5](https://tools.ietf.org/html/rfc3986#section-3.5).
    pub id: Option<Id>,

    /// The locale identifier for the conclusion.
    pub lang: Option<Lang>,

    /// The list of references to the sources of related to this conclusion.
    /// Note that the sources referenced from conclusions are also considered
    /// to be sources of the entities that contain them. For example, a source
    /// associated with the `Name` of a `Person` is also source for the
    /// `Person`.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    /// A reference to the analysis document explaining the analysis that went
    /// into this conclusion. If provided, MUST resolve to an instance of
    /// [Document](crate::Document) of type
    /// [Analysis](crate::DocumentType::Analysis).
    // TODO: Validate this at compile time somehow?
    pub analysis: Option<ResourceReference>,

    /// A list of notes about this conclusion.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub notes: Vec<Note>,

    /// The level of confidence the contributor has about the data.
    pub confidence: Option<ConfidenceLevel>,

    /// The attribution of this conclusion.
    /// If not provided, the attribution of the containing data set (e.g. file)
    /// of the conclusion is assumed.
    pub attribution: Option<Attribution>,
}

impl ConclusionData {
    pub fn new() -> Self {
        Self {
            id: None,
            lang: None,
            sources: vec![],
            analysis: None,
            notes: vec![],
            confidence: None,
            attribution: None,
        }
    }
}

/// Levels of confidence.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum ConfidenceLevel {
    /// The contributor has a high degree of confidence that the assertion is
    /// true.
    High,
    /// The contributor has a medium degree of confidence that the assertion is
    /// true.
    Medium,
    /// The contributor has a low degree of confidence that the assertion is
    /// true.
    Low,
    Custom(Uri),
}

impl From<EnumAsString> for ConfidenceLevel {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/High" => Self::High,
            "http://gedcomx.org/Medium" => Self::Medium,
            "http://gedcomx.org/Low" => Self::Low,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for ConfidenceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::High => write!(f, "http://gedcomx.org/High"),
            Self::Medium => write!(f, "http://gedcomx.org/Medium"),
            Self::Low => write!(f, "http://gedcomx.org/Low"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestData;

    #[test]
    fn json_deserialize() {
        let data = TestData::new();

        let json = r#"{
            "id" : "local_id",
            "lang" : "en",
            "sources" : [ {
                "description" : "SD-1",
                "descriptionId" : "Description id of the target source",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                },
                "qualifiers" : [ { "name" : "http://gedcomx.org/RectangleRegion", "value" : "rectangle region value" } ]          
            }],
            "analysis" : {
              "resource" : "http://identifier/for/analysis/document"
            },
            "notes" : [ {
                "lang" : "en",
                "subject" : "subject",
                "text" : "This is a note",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                }        
            } ],
            "confidence" : "http://gedcomx.org/High",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            }  
        }"#;

        let conclusion_data: ConclusionData = serde_json::from_str(json).unwrap();

        assert_eq!(conclusion_data, data.conclusion_data)
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{}"#;

        let conclusion_data: ConclusionData = serde_json::from_str(json).unwrap();
        assert_eq!(conclusion_data, ConclusionData::new())
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let conclusion_data = data.conclusion_data;

        let json = serde_json::to_string(&conclusion_data).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let conclusion_data = ConclusionData::new();
        let json = serde_json::to_string(&conclusion_data).unwrap();
        assert_eq!(json, r#"{}"#);
    }
}

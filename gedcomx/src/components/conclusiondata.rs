use crate::components::{Attribution, Id, Note, ResourceReference, SourceReference, Uri};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::EnumAsString;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct ConclusionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub notes: Vec<Note>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<ConfidenceLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum ConfidenceLevel {
    High,
    Medium,
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
    use crate::components::TestData;

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

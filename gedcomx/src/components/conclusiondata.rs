use crate::components::{Attribution, Id, Note, ResourceReference, SourceReference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct ConclusionData {
    pub id: Id,

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
    pub fn new(id: Id) -> Self {
        Self {
            id,
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
pub enum ConfidenceLevel {
    #[serde(rename = "http://gedcomx.org/High")]
    High,

    #[serde(rename = "http://gedcomx.org/Medium")]
    Medium,

    #[serde(rename = "http://gedcomx.org/Low")]
    Low,
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
        let json = r#"{"id": "test id"}"#;

        let conclusion_data: ConclusionData = serde_json::from_str(json).unwrap();
        assert_eq!(conclusion_data, ConclusionData::new("test id".to_string()))
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
        let conclusion_data = ConclusionData::new("test id".to_string());
        let json = serde_json::to_string(&conclusion_data).unwrap();
        assert_eq!(json, r#"{"id":"test id"}"#);
    }
}
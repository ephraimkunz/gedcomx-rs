use crate::components::{Attribution, Id, Qualifier, Uri};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SourceReference {
    pub description: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_id: Option<Id>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub qualifiers: Vec<Qualifier>,
}

impl SourceReference {
    pub fn new(description: Uri) -> Self {
        Self {
            description,
            description_id: None,
            attribution: None,
            qualifiers: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum SourceReferenceQualifier {
    CharacterRegion,
    RectangleRegion,
    TimeRegion,
}

impl SourceReferenceQualifier {
    pub fn name(&self) -> &str {
        match self {
            Self::CharacterRegion => "http://gedcomx.org/CharacterRegion",
            Self::RectangleRegion => "http://gedcomx.org/RectangleRegion",
            Self::TimeRegion => "http://gedcomx.org/TimeRegion",
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
            "description" : "SD-1",
            "descriptionId" : "Description id of the target source",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            },
            "qualifiers" : [ { "name" : "http://gedcomx.org/RectangleRegion", "value" : "rectangle region value" } ]          
        }"#;

        let source_reference: SourceReference = serde_json::from_str(json).unwrap();

        assert_eq!(
            source_reference,
            SourceReference {
                description: Uri::from("SD-1"),
                description_id: Some("Description id of the target source".to_string()),
                attribution: data.attribution(),
                qualifiers: vec![Qualifier {
                    name: SourceReferenceQualifier::RectangleRegion.into(),
                    value: Some("rectangle region value".to_string())
                }],
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "description" : "SD-1"
        }"#;

        let source_reference: SourceReference = serde_json::from_str(json).unwrap();
        assert_eq!(source_reference, SourceReference::new(Uri::from("SD-1")))
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let source_reference = SourceReference {
            description: Uri::from("SD-1"),
            description_id: Some("Description id of the target source".to_string()),
            attribution: data.attribution(),
            qualifiers: vec![Qualifier {
                name: SourceReferenceQualifier::RectangleRegion.into(),
                value: Some("rectangle region value".to_string()),
            }],
        };

        let json = serde_json::to_string(&source_reference).unwrap();
        assert_eq!(
            json,
            r#"{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}"#
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let source_reference = SourceReference::new(Uri::from("SD-1"));

        let json = serde_json::to_string(&source_reference).unwrap();
        assert_eq!(json, r#"{"description":"SD-1"}"#);
    }
}

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{ConclusionData, EvidenceReference, Identifier, SourceReference};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[non_exhaustive]
pub struct SubjectData {
    #[serde(flatten)]
    pub conclusion: ConclusionData,

    pub extracted: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub evidence: Vec<EvidenceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub media: Vec<SourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub identifiers: Vec<Identifier>,
}

impl SubjectData {
    pub fn new(conclusion: ConclusionData) -> Self {
        Self {
            conclusion,
            extracted: None,
            evidence: vec![],
            media: vec![],
            identifiers: vec![],
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
            "extracted" : false,
            "evidence" : [ {
                "resource" : "S-1",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                }        
            } ],
            "media" : [ {
                "description" : "SD-1",
                "descriptionId" : "Description id of the target source",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                },
                "qualifiers" : [ { "name" : "http://gedcomx.org/RectangleRegion", "value" : "rectangle region value" } ]          
            } ],

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

        let subject_data: SubjectData = serde_json::from_str(json).unwrap();

        assert_eq!(subject_data, data.subject_data)
    }

    #[test]
    fn json_deserialize_optional_fields() {
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

        let subject_data: SubjectData = serde_json::from_str(json).unwrap();

        assert_eq!(
            subject_data,
            SubjectData {
                extracted: None,
                evidence: vec![],
                media: vec![],
                identifiers: vec![],
                conclusion: data.conclusion_data,
            }
        )
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();
        let subject_data = data.subject_data;
        let json = serde_json::to_string(&subject_data).unwrap();

        assert_eq!(json, "{\"id\":\"local_id\",\"lang\":\"en\",\"sources\":[{\"description\":\"SD-1\",\"descriptionId\":\"Description id of the target source\",\"attribution\":{\"contributor\":{\"resource\":\"A-1\"},\"modified\":1394175600000},\"qualifiers\":[{\"name\":\"http://gedcomx.org/RectangleRegion\",\"value\":\"rectangle region value\"}]}],\"analysis\":{\"resource\":\"http://identifier/for/analysis/document\"},\"notes\":[{\"lang\":\"en\",\"subject\":\"subject\",\"text\":\"This is a note\",\"attribution\":{\"contributor\":{\"resource\":\"A-1\"},\"modified\":1394175600000}}],\"confidence\":\"http://gedcomx.org/High\",\"attribution\":{\"contributor\":{\"resource\":\"A-1\"},\"modified\":1394175600000},\"extracted\":false,\"evidence\":[{\"resource\":\"S-1\",\"attribution\":{\"contributor\":{\"resource\":\"A-1\"},\"modified\":1394175600000}}],\"media\":[{\"description\":\"SD-1\",\"descriptionId\":\"Description id of the target source\",\"attribution\":{\"contributor\":{\"resource\":\"A-1\"},\"modified\":1394175600000},\"qualifiers\":[{\"name\":\"http://gedcomx.org/RectangleRegion\",\"value\":\"rectangle region value\"}]}]}")
    }

    #[test]
    fn json_serialize_optional_fields() {
        let data = TestData::new();
        let subject_data = SubjectData {
            extracted: None,
            evidence: vec![],
            media: vec![],
            identifiers: vec![],
            conclusion: data.conclusion_data,
        };
        let json = serde_json::to_string(&subject_data).unwrap();

        assert_eq!(json, "{\"id\":\"local_id\",\"lang\":\"en\",\"sources\":[{\"description\":\"SD-1\",\"descriptionId\":\"Description id of the target source\",\"attribution\":{\"contributor\":{\"resource\":\"A-1\"},\"modified\":1394175600000},\"qualifiers\":[{\"name\":\"http://gedcomx.org/RectangleRegion\",\"value\":\"rectangle region value\"}]}],\"analysis\":{\"resource\":\"http://identifier/for/analysis/document\"},\"notes\":[{\"lang\":\"en\",\"subject\":\"subject\",\"text\":\"This is a note\",\"attribution\":{\"contributor\":{\"resource\":\"A-1\"},\"modified\":1394175600000}}],\"confidence\":\"http://gedcomx.org/High\",\"attribution\":{\"contributor\":{\"resource\":\"A-1\"},\"modified\":1394175600000}}")
    }
}

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{ConclusionData, EvidenceReference, Identifier, SourceReference};

/// The abstract concept of a genealogical subject.
///
/// A "subject" is something with a unique and intrinsic identity, such as a
/// person or a location on the surface of the earth. We identify that subject
/// in time and space using various supporting conclusions. For example, a
/// person is a subject with supporting conclusions such as name, birth, sex,
/// etc. We aggregate these supporting conclusions to form an apparently-unique
/// identity by which we can distinguish our subject from all other possible
/// subjects.
///
/// Note that a subject is itself a conclusion and can be used as a supporting
/// conclusion for other subjects (via the evidence property). However, not all
/// supporting conclusions are subjects. Researchers may research and debate a
/// fact (e.g. where it took place, when it took place, the spelling of the
/// name, etc.), but it is always within the context of a subject (e.g. where
/// was the person born, when was the person born, how should the person's name
/// be spelled).

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[non_exhaustive]
pub struct SubjectData {
    #[serde(flatten)]
    pub conclusion: ConclusionData,

    /// Whether this subject is to be constrained as an extracted conclusion.
    pub extracted: Option<bool>,

    /// References to other subjects that support this subject.
    ///
    /// If provided, each reference MUST resolve to an instance of subject of
    /// the same type as this instance (e.g., if the subject is an instance of
    /// Person, all of its evidence references must resolve to instances of
    /// Person).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub evidence: Vec<EvidenceReference>,

    /// References to multimedia resources for this subject, such as photos or
    /// videos, intended to provide additional context or illustration for the
    /// subject and not considered evidence supporting the identity of the
    /// subject or its supporting conclusions.
    ///
    /// Media references SHOULD be ordered by priority such that applications
    /// that wish to display a single media item (such as an image) MAY choose
    /// the first applicable media reference. Note that the SourceReference is
    /// used for multimedia references and therefore MUST resolve to a
    /// SourceDescription of the resource, which in turn provides a reference to
    /// the resource itself.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub media: Vec<SourceReference>,

    /// A list of identifiers for the subject.
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

// TODO: Probably want a builder for this.

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

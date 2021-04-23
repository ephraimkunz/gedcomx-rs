use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, Date, Id, Lang, Note, ResourceReference, SourceReference, Uri,
};

/// A role of a person in a group.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct GroupRole {
    /// An identifier for the conclusion data. The id is to be used as a "fragment identifier" as defined by [RFC 3986, Section 3.5](https://tools.ietf.org/html/rfc3986#section-3.5).
    #[yaserde(attribute)]
    pub id: Option<Id>,

    /// The locale identifier for the conclusion.
    #[yaserde(attribute, prefix = "xml")]
    pub lang: Option<Lang>,

    /// The list of references to the sources of related to this conclusion.
    /// Note that the sources referenced from conclusions are also considered
    /// to be sources of the entities that contain them. For example, a source
    /// associated with the `Name` of a `Person` is also source for the
    /// `Person`.
    #[yaserde(rename = "source", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    /// A reference to the analysis document explaining the analysis that went
    /// into this conclusion. If provided, MUST resolve to an instance of
    /// [Document](crate::Document) of type
    /// [Analysis](crate::DocumentType::Analysis).
    // TODO: Validate this at compile time somehow?
    #[yaserde(prefix = "gx")]
    pub analysis: Option<ResourceReference>,

    /// A list of notes about this conclusion.
    #[yaserde(rename = "note", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub notes: Vec<Note>,

    /// The level of confidence the contributor has about the data.
    #[yaserde(attribute)]
    pub confidence: Option<ConfidenceLevel>,

    /// The attribution of this conclusion.
    /// If not provided, the attribution of the containing data set (e.g. file)
    /// of the conclusion is assumed.
    #[yaserde(prefix = "gx")]
    pub attribution: Option<Attribution>,

    /// Reference to the group participant.	MUST resolve to an instance of
    /// [`Person`](crate::Person).
    // TODO: Enforce in type system?
    pub person: ResourceReference,

    /// The date of applicability of the role.
    pub date: Option<Date>,

    /// Details about the role of the participant in the group.
    pub details: Option<String>,

    /// The participant's role.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub group_role_type: Option<GroupRoleType>,
}

impl GroupRole {
    pub fn new(
        id: Option<Id>,
        lang: Option<Lang>,
        sources: Vec<SourceReference>,
        analysis: Option<ResourceReference>,
        notes: Vec<Note>,
        confidence: Option<ConfidenceLevel>,
        attribution: Option<Attribution>,
        person: ResourceReference,
    ) -> Self {
        Self {
            id,
            lang,
            sources,
            analysis,
            notes,
            confidence,
            attribution,
            person,
            date: None,
            details: None,
            group_role_type: None,
        }
    }
}

/// Group role types.
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(untagged)]
pub enum GroupRoleType {
    Custom(Uri),
}

impl Default for GroupRoleType {
    fn default() -> Self {
        Self::Custom(Uri::default())
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
            "person" : {
                "resource" : "http://identifier/for/person/1"
            },
            "type" : "testType",
            "date" : {
                "original" : "the original text"
            },
            "details" : "details",

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

        let group_role: GroupRole = serde_json::from_str(json).unwrap();

        assert_eq!(
            group_role,
            GroupRole {
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
                date: Some(Date {
                    original: Some("the original text".to_string()),
                    formal: None
                }),
                group_role_type: Some(GroupRoleType::Custom("testType".into())),
                details: Some("details".to_string()),
                person: ResourceReference::from("http://identifier/for/person/1")
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let data = TestData::new();

        let json = r#"{   
            "person" : {
                "resource" : "http://identifier/for/person/1"
            },
            
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

        let group_role: GroupRole = serde_json::from_str(json).unwrap();

        assert_eq!(
            group_role,
            GroupRole {
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
                date: None,
                group_role_type: None,
                details: None,
                person: ResourceReference::from("http://identifier/for/person/1")
            }
        )
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let group_role = GroupRole {
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
            date: Some(Date {
                original: Some("the original text".to_string()),
                formal: None,
            }),
            group_role_type: Some(GroupRoleType::Custom("testType".into())),
            details: Some("details".to_string()),
            person: ResourceReference::from("http://identifier/for/person/1"),
        };

        let json = serde_json::to_string(&group_role).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"person":{"resource":"http://identifier/for/person/1"},"date":{"original":"the original text"},"details":"details","type":"testType"}"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let data = TestData::new();

        let group_role = GroupRole {
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
            date: None,
            group_role_type: None,
            details: None,
            person: ResourceReference::from("http://identifier/for/person/1"),
        };

        let json = serde_json::to_string(&group_role).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"person":{"resource":"http://identifier/for/person/1"}}"#
        )
    }
}

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{ConclusionData, Date, ResourceReference, Uri};

/// A role of a person in a group.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct GroupRole {
    #[yaserde(flatten)]
    #[serde(flatten)]
    pub conclusion: ConclusionData,

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
    pub fn new(conclusion: ConclusionData, person: ResourceReference) -> Self {
        Self {
            conclusion,
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
                conclusion: data.conclusion_data,
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
                conclusion: data.conclusion_data,
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
            conclusion: data.conclusion_data,
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
            conclusion: data.conclusion_data,
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

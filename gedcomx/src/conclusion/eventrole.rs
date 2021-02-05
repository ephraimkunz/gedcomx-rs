use crate::{Conclusion, ConclusionData, EnumAsString, Person, ResourceReference, Result, Uri};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{convert::TryInto, fmt};

/// A role played in an event by a person.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct EventRole {
    #[serde(flatten)]
    pub conclusion: ConclusionData,

    /// Reference to the event participant.
    ///
    /// MUST resolve to an instance of [`Person`](crate::Person).
    // TODO: Enforce this with type system?
    pub person: ResourceReference,

    /// The participant's role.
    #[serde(rename = "type")]
    pub event_role_type: Option<EventRoleType>,

    /// Details about the role of participant in the event.
    pub details: Option<String>,
}

impl EventRole {
    pub fn new(
        conclusion: ConclusionData,
        person: ResourceReference,
        event_role_type: Option<EventRoleType>,
        details: Option<String>,
    ) -> Self {
        Self {
            conclusion,
            person,
            event_role_type,
            details,
        }
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `person` has no `id` set.
    pub fn builder(person: &Person) -> Result<EventRoleBuilder> {
        EventRoleBuilder::new(person)
    }
}

impl Conclusion for EventRole {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("EventRole")
    }
}

pub struct EventRoleBuilder(EventRole);

impl EventRoleBuilder {
    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a conversion into [`ResourceReference`](crate::SourceReference) fails.
    /// This happens if `person` has no `id` set.
    pub(crate) fn new(person: &Person) -> Result<Self> {
        Ok(Self(EventRole {
            person: person.try_into()?,
            ..EventRole::default()
        }))
    }

    conclusion_builder_functions!(EventRole);

    pub fn event_role_type(&mut self, event_role_type: EventRoleType) -> &mut Self {
        self.0.event_role_type = Some(event_role_type);
        self
    }

    pub fn details<I: Into<String>>(&mut self, details: I) -> &mut Self {
        self.0.details = Some(details.into());
        self
    }

    pub fn build(&self) -> EventRole {
        EventRole::new(
            self.0.conclusion.clone(),
            self.0.person.clone(),
            self.0.event_role_type.clone(),
            self.0.details.clone(),
        )
    }
}

/// Standard event roles.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum EventRoleType {
    /// The person is the principal person of the event.
    ///
    /// For example, the principal of a birth event is the person that was born.
    Principal,

    /// A participant in the event.
    Participant,

    /// A person officiating the event.
    Official,

    /// A witness of the event.
    Witness,
    Custom(Uri),
}

impl From<EnumAsString> for EventRoleType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Principal" => Self::Principal,
            "http://gedcomx.org/Participant" => Self::Participant,
            "http://gedcomx.org/Official" => Self::Official,
            "http://gedcomx.org/Witness" => Self::Witness,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for EventRoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Principal => write!(f, "http://gedcomx.org/Principal"),
            Self::Participant => write!(f, "http://gedcomx.org/Participant"),
            Self::Official => write!(f, "http://gedcomx.org/Official"),
            Self::Witness => write!(f, "http://gedcomx.org/Witness"),
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
            "person" : {
              "resource" : "http://identifier/for/person/1"
            },
            "type" : "http://gedcomx.org/Witness",
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

        let event_role: EventRole = serde_json::from_str(json).unwrap();

        assert_eq!(
            event_role,
            EventRole {
                conclusion: data.conclusion_data,
                event_role_type: Some(EventRoleType::Witness),
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

        let event_role: EventRole = serde_json::from_str(json).unwrap();

        assert_eq!(
            event_role,
            EventRole {
                conclusion: data.conclusion_data,
                event_role_type: None,
                details: None,
                person: ResourceReference::from("http://identifier/for/person/1")
            }
        )
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let event_role = EventRole {
            conclusion: data.conclusion_data,
            event_role_type: Some(EventRoleType::Witness),
            details: Some("details".to_string()),
            person: ResourceReference::from("http://identifier/for/person/1"),
        };

        let json = serde_json::to_string(&event_role).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"person":{"resource":"http://identifier/for/person/1"},"type":"http://gedcomx.org/Witness","details":"details"}"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let data = TestData::new();

        let event_role = EventRole {
            conclusion: data.conclusion_data,
            event_role_type: None,
            details: None,
            person: ResourceReference::from("http://identifier/for/person/1"),
        };

        let json = serde_json::to_string(&event_role).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"person":{"resource":"http://identifier/for/person/1"}}"#
        )
    }
}

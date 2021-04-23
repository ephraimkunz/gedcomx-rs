use std::{convert::TryInto, fmt};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, EnumAsString, Id, Lang, Note, Person, ResourceReference, Result,
    SourceReference, Uri,
};

/// A role played in an event by a person.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct EventRole {
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

    /// Reference to the event participant.
    ///
    /// MUST resolve to an instance of [`Person`](crate::Person).
    // TODO: Enforce this with type system?
    pub person: ResourceReference,

    /// The participant's role.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub event_role_type: Option<EventRoleType>,

    /// Details about the role of participant in the event.
    pub details: Option<String>,
}

impl EventRole {
    pub fn new(
        id: Option<Id>,
        lang: Option<Lang>,
        sources: Vec<SourceReference>,
        analysis: Option<ResourceReference>,
        notes: Vec<Note>,
        confidence: Option<ConfidenceLevel>,
        attribution: Option<Attribution>,
        person: ResourceReference,
        event_role_type: Option<EventRoleType>,
        details: Option<String>,
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
            event_role_type,
            details,
        }
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `person` has no `id` set.
    pub fn builder(person: &Person) -> Result<EventRoleBuilder> {
        EventRoleBuilder::new(person)
    }
}

pub struct EventRoleBuilder(EventRole);

impl EventRoleBuilder {
    conclusion_builder_functions!(EventRole);

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::SourceReference) fails.
    /// This happens if `person` has no `id` set.
    pub(crate) fn new(person: &Person) -> Result<Self> {
        Ok(Self(EventRole {
            person: person.try_into()?,
            ..EventRole::default()
        }))
    }

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
            self.0.id.clone(),
            self.0.lang.clone(),
            self.0.sources.clone(),
            self.0.analysis.clone(),
            self.0.notes.clone(),
            self.0.confidence.clone(),
            self.0.attribution.clone(),
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

impl_enumasstring_yaserialize_yadeserialize!(EventRoleType, "EventRoleType");

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

impl Default for EventRoleType {
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
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
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
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
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
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
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
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
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

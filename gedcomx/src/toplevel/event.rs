use crate::{Conclusion, ConclusionData, Date, EventRole, PlaceReference, Subject, SubjectData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct Event {
    #[serde(flatten)]
    pub subject: SubjectData,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub event_type: Option<EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<PlaceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub roles: Vec<EventRole>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum EventType {
    Adoption,
    Birth,
    Burial,
    Census,
    Christening,
    Death,
    Divorce,
    Marriage, // TODO: See https://github.com/FamilySearch/gedcomx/blob/master/specifications/event-types-specification.md for more.
}

impl Conclusion for Event {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Subject for Event {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

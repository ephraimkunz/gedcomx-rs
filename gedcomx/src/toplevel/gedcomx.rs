use crate::{
    Agent, Attribution, Document, Event, Group, Id, Lang, Person, PlaceDescription, Relationship,
    SourceDescription, Uri,
};
use serde::{Deserialize, Serialize};

// This struct holds the "real copies" of all the structs that will be serialized to a given format.
// Other structs may hold refs to, for example, SourceDescription, keyed off the ids.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Gedcomx {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub persons: Vec<Person>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub relationships: Vec<Relationship>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub source_descriptions: Vec<SourceDescription>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub agents: Vec<Agent>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub events: Vec<Event>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub documents: Vec<Document>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub places: Vec<PlaceDescription>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub groups: Vec<Group>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Uri>,
}

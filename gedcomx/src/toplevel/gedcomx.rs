use crate::{
    Agent, Attribution, Document, Event, Group, Id, Lang, Person, PlaceDescription, Relationship,
    SourceDescription, Uri,
};
use serde::{Deserialize, Serialize};

// This struct holds the "real copies" of all the structs that will be serialized to a given format.
// Other structs may hold refs to, for example, SourceDescription, keyed off the ids.
#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
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

impl Gedcomx {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Option<Id>,
        lang: Option<Lang>,
        attribution: Option<Attribution>,
        persons: Vec<Person>,
        relationships: Vec<Relationship>,
        source_descriptions: Vec<SourceDescription>,
        agents: Vec<Agent>,
        events: Vec<Event>,
        documents: Vec<Document>,
        places: Vec<PlaceDescription>,
        groups: Vec<Group>,
        description: Option<Uri>,
    ) -> Self {
        Self {
            id,
            lang,
            attribution,
            persons,
            relationships,
            source_descriptions,
            agents,
            events,
            documents,
            places,
            groups,
            description,
        }
    }

    pub fn builder() -> GedcomxBuilder {
        GedcomxBuilder::new()
    }
}

pub struct GedcomxBuilder(Gedcomx);

impl GedcomxBuilder {
    pub(crate) fn new() -> Self {
        Self(Gedcomx::default())
    }

    pub fn agent(&mut self, agent: Agent) -> &mut Self {
        self.0.agents.push(agent);
        self
    }

    pub fn person(&mut self, person: Person) -> &mut Self {
        self.0.persons.push(person);
        self
    }

    pub fn relationship(&mut self, relationship: Relationship) -> &mut Self {
        self.0.relationships.push(relationship);
        self
    }

    pub fn document(&mut self, document: Document) -> &mut Self {
        self.0.documents.push(document);
        self
    }

    pub fn attribution(&mut self, atribution: Attribution) -> &mut Self {
        self.0.attribution = Some(atribution);
        self
    }

    pub fn source_description(&mut self, source_description: SourceDescription) -> &mut Self {
        self.0.source_descriptions.push(source_description);
        self
    }

    pub fn build(&self) -> Gedcomx {
        Gedcomx::new(
            self.0.id.clone(),
            self.0.lang.clone(),
            self.0.attribution.clone(),
            self.0.persons.clone(),
            self.0.relationships.clone(),
            self.0.source_descriptions.clone(),
            self.0.agents.clone(),
            self.0.events.clone(),
            self.0.documents.clone(),
            self.0.places.clone(),
            self.0.groups.clone(),
            self.0.description.clone(),
        )
    }
}

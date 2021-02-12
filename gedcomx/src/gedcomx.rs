use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Agent, Attribution, Document, Event, Group, Id, Lang, Person, PlaceDescription, Relationship,
    SourceDescription, Uri,
};

/// A container for a set of GEDCOM X data.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Gedcomx {
    /// An identifier for the data set.
    pub id: Option<Id>,

    /// The locale identifier for the data set.
    pub lang: Option<Lang>,

    /// The attribution of this data set.
    pub attribution: Option<Attribution>,

    /// The list of persons contained in the data set.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub persons: Vec<Person>,

    /// The list of relationships contained in the data set.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub relationships: Vec<Relationship>,

    /// The list of source descriptions contained in the data set.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub source_descriptions: Vec<SourceDescription>,

    /// The list of agents contained in the data set.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub agents: Vec<Agent>,

    /// The list of events contained in the data set.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub events: Vec<Event>,

    /// The list of documents contained in the data set.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub documents: Vec<Document>,

    /// The list of places contained in the data set.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub places: Vec<PlaceDescription>,

    /// The list of groups contained in the data set.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub groups: Vec<Group>,

    /// Reference to the description of this data set.
    ///
    /// If provided, MUST resolve to an instance of SourceDescription.
    // TODO: Enforce
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

    pub fn persons(&mut self, persons: Vec<Person>) -> &mut Self {
        self.0.persons = persons;
        self
    }

    pub fn relationship(&mut self, relationship: Relationship) -> &mut Self {
        self.0.relationships.push(relationship);
        self
    }

    pub fn relationships(&mut self, relationships: Vec<Relationship>) -> &mut Self {
        self.0.relationships = relationships;
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

    pub fn event(&mut self, event: Event) -> &mut Self {
        self.0.events.push(event);
        self
    }

    pub fn source_description(&mut self, source_description: SourceDescription) -> &mut Self {
        self.0.source_descriptions.push(source_description);
        self
    }

    pub fn source_descriptions(
        &mut self,
        source_descriptions: Vec<SourceDescription>,
    ) -> &mut Self {
        self.0.source_descriptions = source_descriptions;
        self
    }

    pub fn agents(&mut self, agents: Vec<Agent>) -> &mut Self {
        self.0.agents = agents;
        self
    }

    pub fn places(&mut self, places: Vec<PlaceDescription>) -> &mut Self {
        self.0.places = places;
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

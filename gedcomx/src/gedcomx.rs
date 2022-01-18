use std::convert::TryInto;

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Agent, Attribution, Document, Event, GedcomxError, Group, Id, Lang, Person, PlaceDescription,
    Relationship, Result, SourceDescription, Uri,
};

/// A container for a set of GEDCOM X data. The top level type in the library.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, YaSerialize, YaDeserialize, Debug, PartialEq, Clone, Default)]
#[non_exhaustive]
#[yaserde(
    rename = "gedcomx",
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[serde(rename_all = "camelCase")]
pub struct Gedcomx {
    /// An identifier for the data set.
    #[yaserde(attribute)]
    pub id: Option<Id>,

    /// The locale identifier for the data set.
    #[yaserde(attribute, prefix = "xml")]
    pub lang: Option<Lang>,

    /// The attribution of this data set.
    pub attribution: Option<Attribution>,

    /// The list of persons contained in the data set.
    #[yaserde(rename = "person", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub persons: Vec<Person>,

    /// The list of relationships contained in the data set.
    #[yaserde(rename = "relationship")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub relationships: Vec<Relationship>,

    /// The list of source descriptions contained in the data set.
    #[yaserde(rename = "sourceDescription")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub source_descriptions: Vec<SourceDescription>,

    /// The list of agents contained in the data set.
    #[yaserde(rename = "agent")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub agents: Vec<Agent>,

    /// The list of events contained in the data set.
    #[yaserde(rename = "event")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub events: Vec<Event>,

    /// The list of documents contained in the data set.
    #[yaserde(rename = "document")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub documents: Vec<Document>,

    /// The list of places contained in the data set.
    #[yaserde(rename = "place")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub places: Vec<PlaceDescription>,

    /// The list of groups contained in the data set.
    #[yaserde(rename = "group")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub groups: Vec<Group>,

    /// Reference to the description of this data set.
    ///
    /// If provided, MUST resolve to an instance of SourceDescription.
    #[yaserde(attribute)]
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

impl Arbitrary for Gedcomx {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut gx = Self::builder()
            .id(Id::arbitrary(g))
            .lang(Lang::arbitrary(g))
            .attribution(Attribution::arbitrary(g))
            .person(Person::arbitrary(g))
            .relationship(Relationship::arbitrary(g))
            .source_description(SourceDescription::arbitrary(g))
            .agent(Agent::arbitrary(g))
            .event(Event::arbitrary(g))
            .document(Document::arbitrary(g))
            .place(PlaceDescription::arbitrary(g))
            .group(Group::arbitrary(g))
            .build();

        gx.description = Some(Uri::arbitrary(g));

        gx
    }
}

pub struct GedcomxBuilder(Gedcomx);

impl GedcomxBuilder {
    pub(crate) fn new() -> Self {
        Self(Gedcomx::default())
    }

    pub fn id<I: Into<Id>>(&mut self, id: I) -> &mut Self {
        self.0.id = Some(id.into());
        self
    }

    pub fn lang<I: Into<Lang>>(&mut self, lang: I) -> &mut Self {
        self.0.lang = Some(lang.into());
        self
    }

    pub fn attribution(&mut self, attribution: Attribution) -> &mut Self {
        self.0.attribution = Some(attribution);
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

    pub fn agent(&mut self, agent: Agent) -> &mut Self {
        self.0.agents.push(agent);
        self
    }

    pub fn agents(&mut self, agents: Vec<Agent>) -> &mut Self {
        self.0.agents = agents;
        self
    }

    pub fn event(&mut self, event: Event) -> &mut Self {
        self.0.events.push(event);
        self
    }

    pub fn events(&mut self, events: Vec<Event>) -> &mut Self {
        self.0.events = events;
        self
    }

    pub fn document(&mut self, document: Document) -> &mut Self {
        self.0.documents.push(document);
        self
    }

    pub fn documents(&mut self, documents: Vec<Document>) -> &mut Self {
        self.0.documents = documents;
        self
    }

    pub fn place(&mut self, place: PlaceDescription) -> &mut Self {
        self.0.places.push(place);
        self
    }

    pub fn places(&mut self, places: Vec<PlaceDescription>) -> &mut Self {
        self.0.places = places;
        self
    }

    pub fn group(&mut self, group: Group) -> &mut Self {
        self.0.groups.push(group);
        self
    }

    pub fn groups(&mut self, groups: Vec<Group>) -> &mut Self {
        self.0.groups = groups;
        self
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`Uri`](crate::Uri) fails.
    /// This happens if `description` has no `id` set.
    pub fn description(&mut self, description: &SourceDescription) -> Result<&mut Self> {
        self.0.description = Some(description.try_into()?);
        Ok(self)
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

// Convenience methods for serializing / deserializing to JSON / XML.
impl Gedcomx {
    /// Serialize the instance as a string of JSON.
    /// # Errors
    ///
    /// Returns `GedcomxError::JSONError` if serialization fails.
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self).map_err(GedcomxError::JSONError)
    }

    /// Serialize the instance as a string of pretty-printed JSON.
    /// # Errors
    ///
    /// Returns `GedcomxError::JSONError` if serialization fails.
    pub fn to_json_string_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(GedcomxError::JSONError)
    }

    /// Serialize the instance as JSON into the IO stream.
    /// # Errors
    ///
    /// Returns `GedcomxError::JSONError` if serialization fails.
    pub fn to_writer_as_json<W: std::io::Write>(&self, writer: W) -> Result<()> {
        serde_json::to_writer(writer, self).map_err(GedcomxError::JSONError)
    }

    /// Serialize the instance as pretty-printed JSON into the IO stream.
    /// # Errors
    ///
    /// Returns `GedcomxError::JSONError` if serialization fails.
    pub fn to_writer_as_json_pretty<W: std::io::Write>(&self, writer: W) -> Result<()> {
        serde_json::to_writer_pretty(writer, self).map_err(GedcomxError::JSONError)
    }

    /// Deserialize an instance of the type from a string of JSON text.
    /// # Errors
    ///
    /// Returns `GedcomxError::JSONError` if deserialization fails.    
    pub fn from_json_str(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(GedcomxError::JSONError)
    }

    /// Deserialize an instance of the type from an IO stream of JSON.
    /// # Errors
    ///
    /// Returns `GedcomxError::JSONError` if deserialization fails.
    pub fn from_json_reader<R: std::io::Read>(rdr: R) -> Result<Self> {
        serde_json::from_reader(rdr).map_err(GedcomxError::JSONError)
    }

    /// Serialize the instance as a string of XML.
    /// # Errors
    ///
    /// Returns `GedcomxError::XMLError` if serialization fails.
    pub fn to_xml_string(&self) -> Result<String> {
        yaserde::ser::to_string(self).map_err(GedcomxError::XMLError)
    }

    /// Serialize the instance as a string of pretty-printed XML.
    /// # Errors
    ///
    /// Returns `GedcomxError::XMLError` if serialization fails.
    pub fn to_xml_string_pretty(&self) -> Result<String> {
        let config = yaserde::ser::Config {
            perform_indent: true,
            ..yaserde::ser::Config::default()
        };
        yaserde::ser::to_string_with_config(self, &config).map_err(GedcomxError::XMLError)
    }

    /// Serialize the instance as XML into the IO stream.
    /// # Errors
    ///
    /// Returns `GedcomxError::XMLError` if serialization fails.
    pub fn to_writer_as_xml<W: std::io::Write>(&self, writer: W) -> Result<()> {
        yaserde::ser::serialize_with_writer(self, writer, &yaserde::ser::Config::default())
            .map(|_| ())
            .map_err(GedcomxError::XMLError)
    }

    /// Serialize the instance as pretty-printed XML into the IO stream.
    /// # Errors
    ///
    /// Returns `GedcomxError::XMLError` if serialization fails.
    pub fn to_writer_as_xml_pretty<W: std::io::Write>(&self, writer: W) -> Result<()> {
        let config = yaserde::ser::Config {
            perform_indent: true,
            ..yaserde::ser::Config::default()
        };
        yaserde::ser::serialize_with_writer(self, writer, &config)
            .map(|_| ())
            .map_err(GedcomxError::XMLError)
    }

    /// Deserialize an instance of the type from a string of XML text.
    /// # Errors
    ///
    /// Returns `GedcomxError::XMLError` if deserialization fails.
    pub fn from_xml_str(s: &str) -> Result<Self> {
        yaserde::de::from_str(s).map_err(GedcomxError::XMLError)
    }

    /// Deserialize an instance of the type from an IO stream of XML.
    /// # Errors
    ///
    /// Returns `GedcomxError::XMLError` if deserialization fails.
    pub fn from_xml_reader<R: std::io::Read>(rdr: R) -> Result<Self> {
        yaserde::de::from_reader(rdr).map_err(GedcomxError::XMLError)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn json_deserialize() {
        let gedcomx: Gedcomx = Gedcomx::from_json_str("{}").unwrap();
        assert_eq!(gedcomx, Gedcomx::default());
    }

    #[test]
    fn xml_deserialize() {
        let gedcomx: Gedcomx = Gedcomx::from_xml_str("<Gedcomx></Gedcomx>").unwrap();
        assert_eq!(gedcomx, Gedcomx::default());
    }

    #[test]
    fn json_serialize() {
        let gedcomx = Gedcomx::default();
        let json = gedcomx.to_json_string().unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn xml_serialize() {
        let gedcomx = Gedcomx::default();
        let json = gedcomx.to_xml_string().unwrap();
        assert_eq!(
            json,
            "<?xml version=\"1.0\" encoding=\"utf-8\"?><gedcomx xmlns=\"http://gedcomx.org/v1/\" \
             />"
        );
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: Gedcomx) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: Gedcomx = serde_json::from_str(&json).unwrap();
        assert_eq!(input, from_json);
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: Gedcomx) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: Gedcomx = yaserde::de::from_str(&xml).unwrap();
        assert_eq!(input, from_xml);
        input == from_xml
    }
}

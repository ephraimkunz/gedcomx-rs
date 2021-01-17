// Originally from https://github.com/Greedeuh/gedcomx/blob/master/src/lib.rs.

use crate::{
    components::{
        Address, Attribution, Conclusion, ConclusionData, Coverage, Date, EventRole, Fact, Gender,
        GroupRole, Id, Identifiable, Identifier, Lang, Name, Note, OnlineAccount, PlaceReference,
        SourceCitation, SourceReference, SourceReferenceQualifier, Subject, SubjectData, TextValue,
        Timestamp,
    },
    ResourceReference,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Uri(String);

impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<&String> for Uri {
    fn from(s: &String) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for Uri {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<SourceReferenceQualifier> for Uri {
    fn from(s: SourceReferenceQualifier) -> Self {
        s.name().into()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Person {
    pub subject: SubjectData,
    pub private: Option<bool>,
    pub gender: Option<Gender>,
    pub names: Vec<Name>,
    pub facts: Vec<Fact>,
}

impl Conclusion for Person {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Subject for Person {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

impl Identifiable for Person {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Relationship {
    Unknow(RelationshipData),
    Couple(RelationshipData),
    ParentChild(RelationshipData),
    EnslavedBy(RelationshipData),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RelationshipData {
    pub subject: SubjectData,
    pub person1: ResourceReference,
    pub person2: ResourceReference,
    pub facts: Vec<ResourceReference>,
}

impl Conclusion for Relationship {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Subject for Relationship {
    fn subject(&self) -> &SubjectData {
        match self {
            Self::Unknow(x) | Self::Couple(x) | Self::ParentChild(x) | Self::EnslavedBy(x) => {
                &x.subject
            }
        }
    }
}

impl Identifiable for Relationship {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SourceDescription {
    pub id: Id,
    pub resource_type: Option<ResourceType>,
    pub citations: Vec<SourceCitation>,
    pub media_type: Option<String>,
    pub about: ResourceReference,
    pub mediator: Option<ResourceReference>,
    pub publisher: Option<ResourceReference>,
    pub sources: Vec<SourceReference>,
    pub analysis: Option<ResourceReference>,
    pub component_of: Option<SourceReference>,
    pub titles: Vec<TextValue>,
    pub notes: Option<Note>,
    pub attribution: Option<Attribution>,
    pub rights: Option<String>,
    pub coverage: Option<Coverage>,
    pub descriptions: Vec<TextValue>,
    pub identifiers: Vec<Identifier>,
    pub created: Option<Timestamp>,
    pub modified: Option<Timestamp>,
    pub repository: Option<Agent>,
}

impl Identifiable for SourceDescription {
    fn id(&self) -> &Id {
        &self.id
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    Collection,
    PhysicalArtifact,
    DigitalArtifact,
    Record,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub id: Id,
    pub identifiers: Vec<String>,
    pub names: Vec<TextValue>,
    pub homepage: Option<String>,
    pub openid: Option<String>,
    pub accounts: Vec<OnlineAccount>,
    pub emails: Vec<String>,
    pub phones: Vec<String>,
    pub addresses: Vec<Address>,
    pub person: Option<ResourceReference>,
}

impl Identifiable for Agent {
    fn id(&self) -> &Id {
        &self.id
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Event {
    Adoption(EventData),
    Birth(EventData),
    Burial(EventData),
    Census(EventData),
    Christening(EventData),
    Death(EventData),
    Divorce(EventData),
    Marriage(EventData),
}

impl Conclusion for Event {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Identifiable for Event {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

impl Subject for Event {
    fn subject(&self) -> &SubjectData {
        match self {
            Self::Adoption(x)
            | Self::Birth(x)
            | Self::Burial(x)
            | Self::Census(x)
            | Self::Christening(x)
            | Self::Death(x)
            | Self::Divorce(x)
            | Self::Marriage(x) => &x.subject,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EventData {
    pub subject: SubjectData,
    pub date: Option<Date>,
    pub place: Option<PlaceReference>,
    pub roles: Vec<EventRole>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Document {
    Analysis(DocumentData),
    Abstract(DocumentData),
    Transcription(DocumentData),
    Translation(DocumentData),
}

impl Conclusion for Document {
    fn conclusion(&self) -> &ConclusionData {
        match self {
            Self::Analysis(x)
            | Self::Abstract(x)
            | Self::Transcription(x)
            | Self::Translation(x) => &x.conclusion,
        }
    }
}

impl Identifiable for Document {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DocumentData {
    pub conclusion: ConclusionData,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PlaceDescription {
    pub subject: SubjectData,
    pub names: NonEmptyVec<TextValue>,
    pub typee: Option<PlaceType>,
    pub place: Option<String>,
    pub jurisdiction: Option<ResourceReference>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub temporal_description: Option<Date>,
    pub spatial_description: Option<Kml>,
}

impl Conclusion for PlaceDescription {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Identifiable for PlaceDescription {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

impl Subject for PlaceDescription {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

pub type NonEmptyVec<T> = Vec<T>;

pub type PlaceType = String;

pub type Kml = String;
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Group {
    pub subject: SubjectData,
    pub names: Vec<TextValue>,
    pub date: Option<Date>,
    pub place: Option<PlaceReference>,
    pub roles: Vec<GroupRole>,
}

impl Conclusion for Group {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Identifiable for Group {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

impl Subject for Group {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

// This struct holds the "real copies" of all the structs that will be serialized to a given format.
// Other structs may hold refs to, for example, SourceDescription, keyed off the ids.
#[derive(Serialize, Debug, PartialEq)]
pub struct Gedcomx {
    pub id: Option<Id>,
    pub lang: Option<Lang>,
    pub attribution: Option<Attribution>,
    pub persons: Vec<Person>,
    pub relationships: Vec<Relationship>,
    pub source_descriptions: Vec<SourceDescription>,
    pub agents: Vec<Agent>,
    pub events: Vec<Event>,
    pub documents: Vec<Document>,
    pub places: Vec<PlaceDescription>,
    pub groups: Vec<Group>,
    pub description: Option<ResourceReference>,
}

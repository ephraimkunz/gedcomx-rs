// Originally from https://github.com/Greedeuh/gedcomx/blob/master/src/lib.rs.

use crate::components::{
    Address, Attribution, Conclusion, ConclusionData, Coverage, Date, EventRole, Fact, Gender,
    GroupRole, Id, Identifiable, Identifier, Lang, Name, Note, OnlineAccount, PlaceReference,
    SourceCitation, SourceReference, Subject, SubjectData, TextValue, Timestamp,
};

#[derive(Debug)]
pub enum Uri<T> {
    Some(Box<T>),
    //    Ref(&'a T),
    Id(String),
}

#[derive(Debug)]
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
#[derive(Debug)]
pub enum Relationship {
    Unknow(RelationshipData),
    Couple(RelationshipData),
    ParentChild(RelationshipData),
    EnslavedBy(RelationshipData),
}

#[derive(Debug)]
pub struct RelationshipData {
    pub subject: SubjectData,
    pub person1: Uri<Person>,
    pub person2: Uri<Person>,
    pub facts: Vec<Uri<Fact>>,
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

#[derive(Debug)]
pub struct SourceDescription {
    pub id: Id,
    pub resource_type: Option<ResourceType>,
    pub citations: Vec<SourceCitation>,
    pub media_type: Option<String>,
    pub about: Uri<Box<dyn Identifiable>>,
    pub mediator: Option<Uri<Agent>>,
    pub publisher: Option<Uri<Agent>>,
    pub sources: Vec<SourceReference>,
    pub analysis: Option<Uri<Document>>,
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
#[derive(Debug)]
pub enum ResourceType {
    Collection,
    PhysicalArtifact,
    DigitalArtifact,
    Record,
}
#[derive(Debug)]
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
    pub person: Option<Uri<Person>>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct EventData {
    pub subject: SubjectData,
    pub date: Option<Date>,
    pub place: Option<PlaceReference>,
    pub roles: Vec<EventRole>,
}
#[derive(Debug)]
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
#[derive(Debug)]
pub struct DocumentData {
    pub conclusion: ConclusionData,
}
#[derive(Debug)]
pub struct PlaceDescription {
    pub subject: SubjectData,
    pub names: NonEmptyVec<TextValue>,
    pub typee: Option<PlaceType>,
    pub place: Option<String>,
    pub jurisdiction: Option<Uri<Box<PlaceDescription>>>,
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
#[derive(Debug)]
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
    pub description: Option<Uri<SourceDescription>>,
}

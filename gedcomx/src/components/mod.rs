mod attribution;
pub use attribution::Attribution;

mod note;
pub use note::Note;

mod textvalue;
pub use textvalue::TextValue;

mod sourcecitation;
pub use sourcecitation::SourceCitation;

mod sourcereference;
pub use sourcereference::{SourceReference, SourceReferenceQualifier};

mod evidencereference;
pub use evidencereference::EvidenceReference;

mod onlineaccount;
pub use onlineaccount::OnlineAccount;

mod address;
pub use address::Address;

use crate::{Agent, Document, Person, PlaceDescription, SourceDescription, Uri};
use serde::{Deserialize, Serialize};

// I think this will need custom JSON serialization / deserialization. Needs to be a map of typee -> [uri].
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Identifier {
    pub uri: Uri,
    pub typee: Option<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum IdentifierType {
    None,
}

struct TestData {
    attribution: Attribution,
}

impl TestData {
    fn new() -> Self {
        let mut attribution = Attribution::new();
        attribution.contributor = Some(ResourceReference::from("A-1"));
        attribution.modified = Some(chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp(1_394_175_600, 0),
            chrono::Utc,
        ));

        Self { attribution }
    }

    fn attribution(&self) -> Option<Attribution> {
        Some(self.attribution.clone())
    }
}

#[cfg(test)]
mod identifier {
    use super::super::*;
}

pub type Id = String;
pub trait Identifiable: std::fmt::Debug {
    fn id(&self) -> &Id;
}

pub trait Conclusion: Identifiable {
    fn conclusion(&self) -> &ConclusionData;
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ConclusionData {
    pub id: Id,
    pub lang: Option<String>,
    pub sources: Vec<SourceReference>,
    pub analysis: Option<ResourceReference>,
    pub notes: Vec<Note>,
    pub confidence: Option<ConfidenceLevel>,
    pub attribution: Option<Attribution>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

pub trait Subject: Conclusion {
    fn subject(&self) -> &SubjectData;
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SubjectData {
    pub conclusion: ConclusionData,
    pub extracted: Option<bool>,
    pub evidence: Vec<EvidenceReference>,
    pub media: Vec<SourceReference>,
    pub identifiers: Vec<Identifier>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male(ConclusionData),
    Female(ConclusionData),
    Unknown(ConclusionData),
    Intersex(ConclusionData),
}

impl Conclusion for Gender {
    fn conclusion(&self) -> &ConclusionData {
        match self {
            Self::Male(x) | Self::Female(x) | Self::Unknown(x) | Self::Intersex(x) => x,
        }
    }
}

impl Identifiable for Gender {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Name {
    None(NameData),
    BirthName(NameData),
    MarriedName(NameData),
    AlsoKnownAs(NameData),
    Nickname(NameData),
    AdoptiveName(NameData),
    FormalName(NameData),
    ReligiousName(NameData),
}

impl Name {
    pub fn data(&self) -> &NameData {
        match self {
            Self::None(x)
            | Self::BirthName(x)
            | Self::MarriedName(x)
            | Self::AlsoKnownAs(x)
            | Self::Nickname(x)
            | Self::AdoptiveName(x)
            | Self::FormalName(x)
            | Self::ReligiousName(x) => x,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NameData {
    pub conclusion: ConclusionData,
    pub name_forms: Vec<NameForm>,
    pub date: Option<Date>,
}

impl Conclusion for Name {
    fn conclusion(&self) -> &ConclusionData {
        &self.data().conclusion
    }
}

impl Identifiable for Name {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Fact {
    Adoption(FactData),
    Birth(FactData),
    Burial(FactData),
    Christening(FactData),
    Death(FactData),
    Residence(FactData),
    Divorce(FactData),
    Marriage(FactData),
}

impl Fact {
    pub fn data(&self) -> &FactData {
        match self {
            Self::Adoption(x)
            | Self::Birth(x)
            | Self::Burial(x)
            | Self::Christening(x)
            | Self::Death(x)
            | Self::Residence(x)
            | Self::Divorce(x)
            | Self::Marriage(x) => x,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FactData {
    pub conclusion: ConclusionData,
    pub date: Option<Date>,
    pub place: Option<PlaceReference>,
    pub value: Option<String>,
    pub qualifiers: Vec<FactQualifier>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FactQualifier {
    Age,
    Cause,
    Religion,
    Transport,
    NonConsensual,
}

impl Conclusion for Fact {
    fn conclusion(&self) -> &ConclusionData {
        &self.data().conclusion
    }
}

impl Identifiable for Fact {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EventRole {
    None(EventRoleData),
    Principal(EventRoleData),
    Participant(EventRoleData),
    Official(EventRoleData),
    Witness(EventRoleData),
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EventRoleData {
    pub person: ResourceReference,
    pub details: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Date {
    pub original: Option<String>,
    pub formal: Option<DateX>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DateX;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PlaceReference {
    pub original: Option<String>,
    pub description_ref: Option<ResourceReference>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum NamePart {
    None(NamePartData),
    Prefix(NamePartData),
    Suffix(NamePartData),
    Given(NamePartData),
    Surname(NamePartData),
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NamePartData {
    pub value: String,
    pub qualifiers: Vec<NamePartQualifier>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum NamePartQualifier {
    Title,
    Primary,
    Secondary,
    Middle,
    Familiar,
    Religious,
    Family,
    Maiden,
    Patronymic,
    Matronymic,
    Geographic,
    Occupational,
    Characteristic,
    Postnom,
    Particle,
    RootName,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NameForm {
    pub lang: Option<Lang>,
    pub full_text: Option<String>,
    pub parts: Vec<NamePart>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Qualifier {
    name: Uri,
    value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Coverage {
    pub spatial: Option<PlaceReference>,
    pub temporal: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GroupRole {
    Unknown(GroupRoleData),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GroupRoleData {
    pub person: ResourceReference,
    pub date: Option<Date>,
    pub details: Option<String>,
}

pub type Lang = String;

pub type Timestamp = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ResourceReference {
    pub resource: Uri,
}

impl From<Agent> for ResourceReference {
    fn from(a: Agent) -> Self {
        Self::from_identifiable(&a)
    }
}

impl From<&str> for ResourceReference {
    fn from(s: &str) -> Self {
        Self { resource: s.into() }
    }
}

impl ResourceReference {
    fn from_identifiable<I: Identifiable>(identifiable: &I) -> Self {
        Self {
            resource: identifiable.id().into(),
        }
    }
}

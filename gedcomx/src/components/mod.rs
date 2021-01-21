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

mod conclusiondata;
pub use conclusiondata::{ConclusionData, ConfidenceLevel};

mod subjectdata;
pub use subjectdata::SubjectData;

mod gender;
pub use gender::{Gender, GenderType};

mod name;
pub use name::{Name, NameForm, NamePart, NamePartQualifier, NamePartType, NameType};

mod fact;
pub use fact::{Fact, FactQualifier, FactType};

mod eventrole;
pub use eventrole::{EventRole, EventRoleType};

mod date;
pub use date::Date;

mod placereference;
pub use placereference::PlaceReference;

mod grouprole;
pub use grouprole::{GroupRole, GroupRoleType};

mod resourcereference;
pub use resourcereference::ResourceReference;

mod qualifier;
pub use qualifier::Qualifier;

mod coverage;
pub use coverage::Coverage;

use crate::Uri;
use serde::{Deserialize, Serialize};
use std::fmt;

// I think this will need custom JSON serialization / deserialization. Needs to be a map of typee -> [uri].
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Identifier {
    pub uri: Uri,
    pub typee: Option<IdentifierType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum IdentifierType {
    Primary,
    Authority,
    Deprecated,
    Custom(Uri),
}

impl From<EnumAsString> for IdentifierType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Primary" => Self::Primary,
            "http://gedcomx.org/Authority" => Self::Authority,
            "http://gedcomx.org/Deprecated" => Self::Deprecated,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for IdentifierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Primary => write!(f, "http://gedcomx.org/Primary"),
            Self::Authority => write!(f, "http://gedcomx.org/Authority"),
            Self::Deprecated => write!(f, "http://gedcomx.org/Deprecated"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[allow(dead_code)]
struct TestData {
    attribution: Attribution,
    source_reference: SourceReference,
    note: Note,
    conclusion_data: ConclusionData,
    evidence_reference: EvidenceReference,
    subject_data: SubjectData,
}

impl TestData {
    #[allow(dead_code)]
    fn new() -> Self {
        let attribution = Attribution {
            contributor: Some(ResourceReference::from("A-1")),
            modified: Some(chrono::DateTime::from_utc(
                chrono::NaiveDateTime::from_timestamp(1_394_175_600, 0),
                chrono::Utc,
            )),
            ..Attribution::default()
        };

        let qualifier = Qualifier {
            name: SourceReferenceQualifier::RectangleRegion.into(),
            value: Some("rectangle region value".to_string()),
        };
        let mut source_reference = SourceReference::builder(Uri::from("SD-1")).build();
        source_reference.description_id = Some("Description id of the target source".to_string());
        source_reference.attribution = Some(attribution.clone());
        source_reference.qualifiers = vec![qualifier];

        let mut note = Note::new("This is a note".to_string());
        note.attribution = Some(attribution.clone());
        note.lang = Some("en".to_string());
        note.subject = Some("subject".to_string());

        let mut conclusion_data = ConclusionData::new();
        conclusion_data.id = Some("local_id".to_string());
        conclusion_data.lang = Some("en".to_string());
        conclusion_data.sources = vec![source_reference.clone()];
        conclusion_data.analysis = Some(ResourceReference::from(
            "http://identifier/for/analysis/document",
        ));
        conclusion_data.notes = vec![note.clone()];
        conclusion_data.confidence = Some(ConfidenceLevel::High);
        conclusion_data.attribution = Some(attribution.clone());

        let mut evidence_reference = EvidenceReference::builder(Uri::from("S-1")).build();
        evidence_reference.attribution = Some(attribution.clone());

        let mut subject_data = SubjectData::new(conclusion_data.clone());
        subject_data.extracted = Some(false);
        subject_data.evidence = vec![evidence_reference.clone()];
        subject_data.media = vec![source_reference.clone()];
        subject_data.identifiers = vec![]; // TODO: Empty until I get this serializing properly.
        subject_data.conclusion = conclusion_data.clone();

        Self {
            attribution,
            source_reference,
            note,
            conclusion_data,
            evidence_reference,
            subject_data,
        }
    }
}

// TODO: Implement custom serializer / deserializer?
pub type Id = String;

pub trait Conclusion {
    fn conclusion(&self) -> &ConclusionData;
}

pub trait Subject: Conclusion {
    fn subject(&self) -> &SubjectData;
}

pub type Lang = String;

pub type Timestamp = chrono::DateTime<chrono::Utc>;

// I can't figure out how to get Serde to properly serialize enums with a bunch of normal variants and then
// one catch-all variant that includes the string from the json, just using attributes. So, rather than write a
// Deserializer / Serializer implementation we'll just serialize to this newtype and then Serde will automatically
//  convert it to the required type.
#[derive(Serialize, Deserialize)]
pub struct EnumAsString(pub String);

impl<T: fmt::Display> From<T> for EnumAsString {
    fn from(t: T) -> Self {
        Self(t.to_string())
    }
}

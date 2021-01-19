use crate::components::EnumAsString;
use crate::{Conclusion, ConclusionData, Fact, ResourceReference, Subject, SubjectData, Uri};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct Relationship {
    #[serde(flatten)]
    pub subject: SubjectData,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub relationship_type: Option<RelationshipType>,

    pub person1: ResourceReference,
    pub person2: ResourceReference,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub facts: Vec<Fact>,
}

impl Relationship {
    pub fn new(
        subject: SubjectData,
        person1: ResourceReference,
        person2: ResourceReference,
    ) -> Self {
        Self {
            subject,
            person1,
            person2,
            relationship_type: None,
            facts: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum RelationshipType {
    AncestorDescendant,
    Couple,
    EnslavedBy,
    Godparent,
    ParentChild,
    Custom(Uri),
}

impl From<EnumAsString> for RelationshipType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/AncestorDescendant" => Self::AncestorDescendant,
            "http://gedcomx.org/Couple" => Self::Couple,
            "http://gedcomx.org/EnslavedBy" => Self::EnslavedBy,
            "http://gedcomx.org/Godparent" => Self::Godparent,
            "http://gedcomx.org/ParentChild" => Self::ParentChild,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::AncestorDescendant => write!(f, "http://gedcomx.org/AncestorDescendant"),
            Self::Couple => write!(f, "http://gedcomx.org/Couple"),
            Self::EnslavedBy => write!(f, "http://gedcomx.org/EnslavedBy"),
            Self::Godparent => write!(f, "http://gedcomx.org/Godparent"),
            Self::ParentChild => write!(f, "http://gedcomx.org/ParentChild"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

impl Conclusion for Relationship {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Subject for Relationship {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

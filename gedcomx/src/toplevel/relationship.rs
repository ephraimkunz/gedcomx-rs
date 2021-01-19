use crate::{Conclusion, ConclusionData, Fact, ResourceReference, Subject, SubjectData};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum RelationshipType {
    #[serde(rename = "http://gedcomx.org/Couple")]
    Couple,

    #[serde(rename = "http://gedcomx.org/ParentChild")]
    ParentChild,

    #[serde(rename = "http://gedcomx.org/EnslavedBy")]
    EnslavedBy,
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

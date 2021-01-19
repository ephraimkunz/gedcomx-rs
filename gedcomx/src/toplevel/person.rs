use crate::{Conclusion, ConclusionData, Fact, Gender, Name, Subject, SubjectData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct Person {
    #[serde(flatten)]
    pub subject: SubjectData,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub names: Vec<Name>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub facts: Vec<Fact>,
}

impl Person {
    pub fn new(subject: SubjectData) -> Self {
        Self {
            subject,
            private: None,
            gender: None,
            names: vec![],
            facts: vec![],
        }
    }
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

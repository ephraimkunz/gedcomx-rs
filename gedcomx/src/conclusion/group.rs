use crate::{
    Conclusion, ConclusionData, Date, GroupRole, PlaceReference, Subject, SubjectData, TextValue,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Group {
    #[serde(flatten)]
    pub subject: SubjectData,

    pub names: Vec<TextValue>, // Must contain at least 1 name

    pub date: Option<Date>,

    pub place: Option<PlaceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub roles: Vec<GroupRole>,
}

impl Conclusion for Group {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.subject_mut().conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("Group")
    }
}

impl Subject for Group {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }

    fn subject_mut(&mut self) -> &mut SubjectData {
        &mut self.subject
    }
}

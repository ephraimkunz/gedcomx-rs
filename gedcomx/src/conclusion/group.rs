use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Conclusion, ConclusionData, Date, GroupRole, PlaceReference, Subject, SubjectData, TextValue,
};

/// A group of of persons.
///
/// The concept of a "group" captures institutional associations between persons
/// that may or may not have direct familial relations between each other.
/// Examples of a group could include plantations, orphanages, or military
/// units.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Group {
    #[serde(flatten)]
    pub subject: SubjectData,

    /// A list of names of the group. The list must contain at least 1 name.
    // TODO: Enforce in type system?
    pub names: Vec<TextValue>,

    /// The date of applicability of the group.
    pub date: Option<Date>,

    /// A reference to the place applicable to this group.
    pub place: Option<PlaceReference>,

    /// Information about how persons were associated with the group.
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

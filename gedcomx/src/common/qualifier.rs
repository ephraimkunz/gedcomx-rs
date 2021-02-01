use crate::Uri;
use serde::{Deserialize, Serialize};

/// Used to supply additional details, annotations, tags, or other qualifying data to a specific data element.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Qualifier {
    /// The name of the qualifier. The name should be an element of a constrained vocabulary and is used to determine meaning of the qualifier.
    pub name: Uri,

    /// The value of the qualifier. Some qualifiers may not have values, indicating that the qualifier is to be treated more like a "tag".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Qualifier {
    pub fn new<U: Into<Uri>, S: Into<String>>(name: U, value: Option<S>) -> Self {
        Self {
            name: name.into(),
            value: value.map(std::convert::Into::into),
        }
    }
}

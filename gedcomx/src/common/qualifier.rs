use crate::Uri;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Qualifier {
    pub name: Uri,

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

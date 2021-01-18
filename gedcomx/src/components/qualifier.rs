use crate::Uri;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Qualifier {
    pub name: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

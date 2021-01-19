use crate::Uri;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct ResourceReference {
    pub resource: Uri,
}

impl From<&str> for ResourceReference {
    fn from(s: &str) -> Self {
        Self { resource: s.into() }
    }
}

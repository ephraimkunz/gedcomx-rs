use crate::{Agent, Identifiable, Uri};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct ResourceReference {
    pub resource: Uri,
}

impl From<Agent> for ResourceReference {
    fn from(a: Agent) -> Self {
        Self::from_identifiable(&a)
    }
}

impl From<&str> for ResourceReference {
    fn from(s: &str) -> Self {
        Self { resource: s.into() }
    }
}

impl ResourceReference {
    fn from_identifiable<I: Identifiable>(identifiable: &I) -> Self {
        Self {
            resource: identifiable.id().into(),
        }
    }
}

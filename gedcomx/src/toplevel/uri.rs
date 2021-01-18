use crate::{FactQualifier, NamePartQualifier, SourceReferenceQualifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Uri(String);

impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<&String> for Uri {
    fn from(s: &String) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for Uri {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<SourceReferenceQualifier> for Uri {
    fn from(s: SourceReferenceQualifier) -> Self {
        s.to_string().into()
    }
}

impl From<NamePartQualifier> for Uri {
    fn from(n: NamePartQualifier) -> Self {
        n.to_string().into()
    }
}

impl From<FactQualifier> for Uri {
    fn from(f: FactQualifier) -> Self {
        f.to_string().into()
    }
}

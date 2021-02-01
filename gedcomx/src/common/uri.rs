use crate::{
    FactQualifier, GedcomxError, NamePartQualifier, PlaceDescription, SourceDescription,
    SourceReferenceQualifier,
};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
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

impl TryFrom<&PlaceDescription> for Uri {
    type Error = GedcomxError;
    fn try_from(pd: &PlaceDescription) -> Result<Self, Self::Error> {
        match &pd.subject.conclusion.id {
            Some(id) => Ok(Self::from(format!("{}{}", "#", id))),
            None => Err(GedcomxError::NoId("PlaceDescription".to_string())),
        }
    }
}

impl TryFrom<&SourceDescription> for Uri {
    type Error = GedcomxError;
    fn try_from(sd: &SourceDescription) -> Result<Self, Self::Error> {
        match &sd.id {
            Some(id) => Ok(Self::from(format!("{}{}", "#", id))),
            None => Err(GedcomxError::NoId("SourceDescription".to_string())),
        }
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

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

use std::{convert::TryFrom, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    FactQualifier, GedcomxError, Id, NamePartQualifier, PlaceDescription, SourceDescription,
    SourceReferenceQualifier,
};

/// Specified by [RFC 3986](https://tools.ietf.org/html/rfc3986).
///
/// GEDCOM X resources use the URI to reference other entities.
/// For example, a GEDCOM X Relationship identifies a person in the relationship
/// by referencing a URI that identifies the person. When a property (such as
/// the person1 property of Relationship) is of data type URI, the value of the
/// property is interpreted as a "URI Reference" as defined by [RFC 3986, section 4](https://tools.ietf.org/html/rfc3986#section-4).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Uri(String);

impl_characters_yaserialize_yadeserialize!(Uri, "Uri");

impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<&String> for Uri {
    fn from(s: &String) -> Self {
        Self(s.clone())
    }
}

impl From<String> for Uri {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Id> for Uri {
    fn from(id: Id) -> Self {
        Self(format!("#{}", id.to_string()))
    }
}

impl From<&Id> for Uri {
    fn from(id: &Id) -> Self {
        Self(format!("#{}", id.to_string()))
    }
}

impl TryFrom<&PlaceDescription> for Uri {
    type Error = GedcomxError;

    fn try_from(pd: &PlaceDescription) -> Result<Self, Self::Error> {
        match &pd.id {
            Some(id) => Ok(id.into()),
            None => Err(GedcomxError::NoId("PlaceDescription".to_string())),
        }
    }
}

impl TryFrom<&SourceDescription> for Uri {
    type Error = GedcomxError;

    fn try_from(sd: &SourceDescription) -> Result<Self, Self::Error> {
        match &sd.id {
            Some(id) => Ok(id.into()),
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
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_source_description() {
        let source_description = SourceDescription::builder().id("test").build();
        let uri = Uri::try_from(&source_description);
        assert_eq!(uri, Ok(Uri::from("#test")))
    }

    #[test]
    fn from_source_description_no_id() {
        let source_description = SourceDescription::builder().build();
        let uri = Uri::try_from(&source_description);
        assert_eq!(
            uri,
            Err(GedcomxError::NoId("SourceDescription".to_string()))
        )
    }

    #[test]
    fn from_place_description() {
        let place_description = PlaceDescription::builder().id("test").build();
        let uri = Uri::try_from(&place_description);
        assert_eq!(uri, Ok(Uri::from("#test")))
    }

    #[test]
    fn from_place_description_no_id() {
        let place_description = PlaceDescription::builder().build();
        let uri = Uri::try_from(&place_description);
        assert_eq!(uri, Err(GedcomxError::NoId("PlaceDescription".to_string())))
    }

    #[test]
    fn from_id() {
        let id = Id::from("hi");
        let id2 = id.clone();
        let uri: Uri = id.into();
        assert_eq!(uri, Uri("#hi".to_string()));

        let uri2: Uri = (&id2).into();
        assert_eq!(uri2, Uri("#hi".to_string()));
    }
}

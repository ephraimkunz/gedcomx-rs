use std::{convert::TryFrom, fmt};

use quickcheck::{Arbitrary, Gen};
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default, Eq)]
pub struct Uri(String);

impl_characters_yaserialize_yadeserialize!(Uri, "Uri");

impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for Uri {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Id> for Uri {
    fn from(id: Id) -> Self {
        Self(format!("#{id}"))
    }
}

impl From<&Id> for Uri {
    fn from(id: &Id) -> Self {
        Self(format!("#{id}"))
    }
}

impl TryFrom<&PlaceDescription> for Uri {
    type Error = GedcomxError;

    fn try_from(pd: &PlaceDescription) -> Result<Self, Self::Error> {
        pd.id
            .as_ref()
            .map_or_else(|| Err(GedcomxError::no_id_error(&pd)), |id| Ok(id.into()))
    }
}

impl TryFrom<&SourceDescription> for Uri {
    type Error = GedcomxError;

    fn try_from(sd: &SourceDescription) -> Result<Self, Self::Error> {
        sd.id
            .as_ref()
            .map_or_else(|| Err(GedcomxError::no_id_error(&sd)), |id| Ok(id.into()))
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

impl Arbitrary for Uri {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(crate::arbitrary_trimmed(g))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::SourceCitation;

    #[test]
    fn from_source_description() {
        let source_description =
            SourceDescription::builder(SourceCitation::new("source_citation", None))
                .id("test")
                .build();
        let uri = Uri::try_from(&source_description);
        assert_eq!(uri.unwrap(), Uri::from("#test"));
    }

    #[test]
    fn from_source_description_no_id() {
        let source_description =
            SourceDescription::builder(SourceCitation::new("source_citation", None)).build();
        let uri = Uri::try_from(&source_description);
        assert_eq!(
            uri.unwrap_err().to_string(),
            GedcomxError::no_id_error(&source_description).to_string()
        );
    }

    #[test]
    fn from_place_description() {
        let place_description = PlaceDescription::builder("name").id("test").build();
        let uri = Uri::try_from(&place_description);
        assert_eq!(uri.unwrap(), Uri::from("#test"));
    }

    #[test]
    fn from_place_description_no_id() {
        let place_description = PlaceDescription::builder("name").build();
        let uri = Uri::try_from(&place_description);
        assert_eq!(
            uri.unwrap_err().to_string(),
            GedcomxError::no_id_error(&place_description).to_string()
        );
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

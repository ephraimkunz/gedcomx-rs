use std::convert::TryFrom;

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Agent, Document, DocumentType, GedcomxError, Person, PlaceDescription, Uri};

/// A generic reference to a resource.
#[derive(
    Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default, Eq,
)]
#[non_exhaustive]
pub struct ResourceReference {
    /// The URI to the resource being referenced.
    #[yaserde(attribute)]
    pub resource: Uri,
}

impl ResourceReference {
    pub fn new(uri: Uri) -> Self {
        Self { resource: uri }
    }
}

impl From<&str> for ResourceReference {
    fn from(s: &str) -> Self {
        Self { resource: s.into() }
    }
}

impl From<String> for ResourceReference {
    fn from(s: String) -> Self {
        Self { resource: s.into() }
    }
}

impl TryFrom<&Agent> for ResourceReference {
    type Error = GedcomxError;

    fn try_from(agent: &Agent) -> Result<Self, Self::Error> {
        agent.id.as_ref().map_or_else(
            || Err(GedcomxError::no_id_error(&agent)),
            |id| {
                Ok(Self {
                    resource: id.into(),
                })
            },
        )
    }
}

impl TryFrom<&Person> for ResourceReference {
    type Error = GedcomxError;

    fn try_from(person: &Person) -> Result<Self, Self::Error> {
        person.id.as_ref().map_or_else(
            || Err(GedcomxError::no_id_error(&person)),
            |id| {
                Ok(Self {
                    resource: id.into(),
                })
            },
        )
    }
}

impl TryFrom<&PlaceDescription> for ResourceReference {
    type Error = GedcomxError;

    fn try_from(place_description: &PlaceDescription) -> Result<Self, Self::Error> {
        place_description.id.as_ref().map_or_else(
            || Err(GedcomxError::no_id_error(&place_description)),
            |id| {
                Ok(Self {
                    resource: id.into(),
                })
            },
        )
    }
}

// The only use of a Document as a ResourceReference is as the analysis field of
// Conclusion and SourceDescription. In both those cases, we care about the
// document being the right type so we'll check it in our try_into impl.
impl TryFrom<&Document> for ResourceReference {
    type Error = GedcomxError;

    fn try_from(document: &Document) -> Result<Self, Self::Error> {
        match (
            &document.id,
            document.document_type.is_none()
                || document.document_type == Some(DocumentType::Analysis),
        ) {
            (Some(id), true) => Ok(Self {
                resource: id.into(),
            }),
            (None, _) => Err(GedcomxError::no_id_error(&document)),
            (_, false) => Err(GedcomxError::WrongDocumentType {
                expected: DocumentType::Analysis,
                actual: document.document_type.as_ref().unwrap().clone(), /* Should never be None
                                                                           * here based on above
                                                                           * match statement. */
            }),
        }
    }
}

impl Arbitrary for ResourceReference {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            resource: Uri::arbitrary(g),
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::GedcomxError;

    #[test]
    fn from_agent() {
        let agent = Agent::builder().id("my id").build();
        let rr: ResourceReference = (&agent).try_into().unwrap();
        let expected = ResourceReference::from("#my id");
        assert_eq!(rr, expected);
    }

    #[test]
    fn from_agent_no_id() {
        let agent = Agent::default();
        let rr: Result<ResourceReference, GedcomxError> = (&agent).try_into();
        let expected = GedcomxError::no_id_error(&agent).to_string();
        assert_eq!(rr.unwrap_err().to_string(), expected);
    }

    #[test]
    fn from_person() {
        let person = Person::builder().id("my id").build();
        let rr: ResourceReference = (&person).try_into().unwrap();
        let expected = ResourceReference::from("#my id");
        assert_eq!(rr, expected);
    }

    #[test]
    fn from_person_no_id() {
        let person = Person::default();
        let rr: Result<ResourceReference, GedcomxError> = (&person).try_into();
        let expected = GedcomxError::no_id_error(&person).to_string();
        assert_eq!(rr.unwrap_err().to_string(), expected);
    }

    #[test]
    fn from_document() {
        let document = Document::builder("")
            .id("my id")
            .document_type(DocumentType::Analysis)
            .build();
        let rr: ResourceReference = (&document).try_into().unwrap();
        let expected = ResourceReference::from("#my id");
        assert_eq!(rr, expected);
    }

    #[test]
    fn from_document_no_type() {
        let document = Document::builder("").id("my id").build();
        let rr: ResourceReference = (&document).try_into().unwrap();
        let expected = ResourceReference::from("#my id");
        assert_eq!(rr, expected);
    }

    #[test]
    fn from_document_no_id() {
        let document = Document::builder("")
            .document_type(DocumentType::Analysis)
            .build();
        let rr: Result<ResourceReference, GedcomxError> = (&document).try_into();
        let expected = GedcomxError::no_id_error(&document).to_string();
        assert_eq!(rr.unwrap_err().to_string(), expected);
    }

    #[test]
    fn from_document_wrong_type() {
        let document = Document::builder("")
            .id("my id")
            .document_type(DocumentType::Abstract)
            .build();
        let rr: Result<ResourceReference, GedcomxError> = (&document).try_into();
        let expected = GedcomxError::WrongDocumentType {
            expected: DocumentType::Analysis,
            actual: DocumentType::Abstract,
        }
        .to_string();
        assert_eq!(rr.unwrap_err().to_string(), expected);
    }

    #[test]
    fn from_document_wrong_type_no_id() {
        let document = Document::default();
        let rr: Result<ResourceReference, GedcomxError> = (&document).try_into();
        let expected = GedcomxError::no_id_error(&document).to_string();
        assert_eq!(rr.unwrap_err().to_string(), expected);
    }
}

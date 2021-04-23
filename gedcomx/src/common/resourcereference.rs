use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Agent, Document, DocumentType, GedcomxError, Person, Uri};

/// A generic reference to a resource.
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct ResourceReference {
    /// The URI to the resource being referenced.
    #[yaserde(attribute)]
    pub resource: Uri,
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
        match &agent.id {
            Some(id) => Ok(Self {
                resource: id.into(),
            }),
            None => Err(GedcomxError::NoId("Agent".to_string())),
        }
    }
}

impl TryFrom<&Person> for ResourceReference {
    type Error = GedcomxError;

    fn try_from(person: &Person) -> Result<Self, Self::Error> {
        match &person.id {
            Some(id) => Ok(Self {
                resource: id.into(),
            }),
            None => Err(GedcomxError::NoId("Person".to_string())),
        }
    }
}

impl TryFrom<&Document> for ResourceReference {
    type Error = GedcomxError;

    fn try_from(document: &Document) -> Result<Self, Self::Error> {
        match (
            &document.id,
            document.document_type == None
                || document.document_type == Some(DocumentType::Analysis),
        ) {
            (Some(id), true) => Ok(Self {
                resource: id.into(),
            }),
            (None, _) => Err(GedcomxError::NoId("Document".to_string())),
            (_, false) => Err(GedcomxError::WrongDocumentType {
                expected: DocumentType::Analysis,
                actual: document.document_type.as_ref().unwrap().clone(), /* Should never be None
                                                                           * here based on above
                                                                           * match statement. */
            }),
        }
    }
}

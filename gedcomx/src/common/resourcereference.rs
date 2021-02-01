use std::convert::TryFrom;

use crate::{Agent, Document, DocumentType, GedcomxError, Person, Uri};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct ResourceReference {
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
                resource: Uri::from(format!("{}{}", "#", id)),
            }),
            None => Err(GedcomxError::NoId("Agent".to_string())),
        }
    }
}

impl TryFrom<&Person> for ResourceReference {
    type Error = GedcomxError;
    fn try_from(person: &Person) -> Result<Self, Self::Error> {
        match &person.subject.conclusion.id {
            Some(id) => Ok(Self {
                resource: Uri::from(format!("{}{}", "#", id)),
            }),
            None => Err(GedcomxError::NoId("Person".to_string())),
        }
    }
}

impl TryFrom<&Document> for ResourceReference {
    type Error = GedcomxError;
    fn try_from(document: &Document) -> Result<Self, Self::Error> {
        match (
            &document.conclusion.id,
            document.document_type == None
                || document.document_type == Some(DocumentType::Analysis),
        ) {
            (Some(id), true) => Ok(Self {
                resource: Uri::from(format!("{}{}", "#", id)),
            }),
            (None, _) => Err(GedcomxError::NoId("Document".to_string())),
            (_, false) => Err(GedcomxError::WrongDocumentType {
                expected: DocumentType::Analysis,
                actual: document.document_type.as_ref().unwrap().clone(), // Should never be None here based on above match statement.
            }),
        }
    }
}

use thiserror::Error;

use crate::DocumentType;

/// An error returned by the library.
#[derive(Error, Debug)]
pub enum GedcomxError {
    /// An object with an `Id` was needed for an operation, but the object had
    /// no id.
    #[error("Can't get a non-None id for `{0}`")]
    NoId(String), // TODO: Maybe should hold the object without id rather than a string?

    /// An object with a certain DocumentType variant was needed for an
    /// operation, but the object had a different type.
    #[error("Wrong DocumentType. Expected: {expected}, Actual: {actual}")]
    WrongDocumentType {
        expected: DocumentType,
        actual: DocumentType,
    },

    /// Error while parsing a string as a Gedcomx date.
    #[error("Error parsing {parsed_string} as date: {error}")]
    DateParse {
        parsed_string: String,
        error: String,
    },

    /// Error while parsing a string as a qualifier name (`FactQualifier`,
    /// `SourceReferenceQualifier`, or `NamePartQualifier`).
    #[error("Error parsing {parsed_string} as qualifier name")]
    QualifierParse { parsed_string: String },

    /// Error returned while attempting to serialize / deserialize as JSON.
    #[error("Error serializing or deserializing JSON")]
    JSONError(#[from] serde_json::Error),

    /// Error returned while attempting to serialize / deserialize as XML.
    #[error("Error serializing or deserializing XML")]
    XMLError(String),
}

use crate::components::EnumAsString;
use crate::{
    Attribution, Coverage, Id, Identifier, Note, ResourceReference, SourceCitation,
    SourceReference, TextValue, Timestamp, Uri,
};
use chrono::serde::ts_milliseconds_option;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct SourceDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceType>,

    pub citations: Vec<SourceCitation>, // Must have at least one.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<Uri>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediator: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_of: Option<SourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub titles: Vec<TextValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Note>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub rights: Vec<ResourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub coverage: Vec<Coverage>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub descriptions: Vec<TextValue>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub identifiers: Vec<Identifier>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option",
        default
    )]
    pub created: Option<Timestamp>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option",
        default
    )]
    pub modified: Option<Timestamp>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option",
        default
    )]
    pub published: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<ResourceReference>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum ResourceType {
    Collection,
    PhysicalArtifact,
    DigitalArtifact,
    Record,
    Custom(Uri),
}

impl From<EnumAsString> for ResourceType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Collection" => Self::Collection,
            "http://gedcomx.org/PhysicalArtifact" => Self::PhysicalArtifact,
            "http://gedcomx.org/DigitalArtifact" => Self::DigitalArtifact,
            "http://gedcomx.org/Record" => Self::Record,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Collection => write!(f, "http://gedcomx.org/Collection"),
            Self::PhysicalArtifact => write!(f, "http://gedcomx.org/PhysicalArtifact"),
            Self::DigitalArtifact => write!(f, "http://gedcomx.org/DigitalArtifact"),
            Self::Record => write!(f, "http://gedcomx.org/Record"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

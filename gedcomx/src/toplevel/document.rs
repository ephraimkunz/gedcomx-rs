use crate::components::EnumAsString;
use crate::{Attribution, Conclusion, ConclusionData, Uri};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(flatten)]
    pub conclusion: ConclusionData,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub document_type: Option<DocumentType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum DocumentType {
    Analysis,
    Abstract,
    Transcription,
    Translation,
    Custom(Uri),
}

impl From<EnumAsString> for DocumentType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Analysis" => Self::Analysis,
            "http://gedcomx.org/Abstract" => Self::Abstract,
            "http://gedcomx.org/Transcription" => Self::Transcription,
            "http://gedcomx.org/Translation" => Self::Translation,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Analysis => write!(f, "http://gedcomx.org/Analysis"),
            Self::Abstract => write!(f, "http://gedcomx.org/Abstract"),
            Self::Transcription => write!(f, "http://gedcomx.org/Transcription"),
            Self::Translation => write!(f, "http://gedcomx.org/Translation"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

impl Conclusion for Document {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }
}

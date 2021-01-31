use crate::components::EnumAsString;
use crate::{Attribution, Conclusion, ConclusionData, Uri};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
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

impl Document {
    pub fn new(
        conclusion: ConclusionData,
        document_type: Option<DocumentType>,
        extracted: Option<bool>,
        text_type: Option<String>,
        text: String,
        attribution: Option<Attribution>,
    ) -> Self {
        Self {
            conclusion,
            document_type,
            extracted,
            text_type,
            text,
            attribution,
        }
    }

    pub fn builder<I: Into<String>>(text: I) -> DocumentBuilder {
        DocumentBuilder::new(text)
    }
}

pub struct DocumentBuilder(Document);

impl DocumentBuilder {
    pub(crate) fn new<I: Into<String>>(text: I) -> Self {
        Self(Document {
            text: text.into(),
            ..Document::default()
        })
    }

    conclusion_builder_functions!(Document);

    pub fn document_type(&mut self, document_type: DocumentType) -> &mut Self {
        self.0.document_type = Some(document_type);
        self
    }

    pub fn build(&self) -> Document {
        Document::new(
            self.0.conclusion.clone(),
            self.0.document_type.clone(),
            self.0.extracted,
            self.0.text_type.clone(),
            self.0.text.clone(),
            self.0.attribution.clone(),
        )
    }
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

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("Document")
    }
}

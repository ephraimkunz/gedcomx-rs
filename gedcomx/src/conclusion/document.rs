use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Attribution, Conclusion, ConclusionData, EnumAsString, Uri};

/// The base conceptual model for genealogical data that are managed as textual
/// documents.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[yaserde(flatten)]
    #[serde(flatten)]
    pub conclusion: ConclusionData,

    /// Enumerated value identifying the type of the document.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub document_type: Option<DocumentType>,

    /// Whether this document is to be constrained as an *xtracted conclusion,
    /// meaning it captures information extracted from a single source.
    pub extracted: Option<bool>,

    /// The type of text in the `text` property.
    ///
    /// If provided, the value MUST be a [valid text type](https://github.com/FamilySearch/gedcomx/blob/master/specifications/conceptual-model-specification.md#text-types). If no value is provided, "plain" is assumed
    // TODO: Newtype for this?
    #[yaserde(rename = "textType")]
    pub text_type: Option<String>,

    /// The text of the document.
    pub text: String,

    /// The attribution of the document.
    ///
    /// If not provided, the attribution of the containing data set (e.g. file)
    /// of the document is assumed.
    // TODO: Should this property even exist? It's also defined on the conclusion data.
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
    conclusion_builder_functions!(Document);

    pub(crate) fn new<I: Into<String>>(text: I) -> Self {
        Self(Document {
            text: text.into(),
            ..Document::default()
        })
    }

    pub fn document_type(&mut self, document_type: DocumentType) -> &mut Self {
        self.0.document_type = Some(document_type);
        self
    }

    pub fn extracted(&mut self, extracted: bool) -> &mut Self {
        self.0.extracted = Some(extracted);
        self
    }

    pub fn text_type<I: Into<String>>(&mut self, text_type: I) -> &mut Self {
        self.0.text_type = Some(text_type.into());
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

/// Document types
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum DocumentType {
    /// The document is an analysis done by a researcher; a genealogical proof
    /// statement is an example of one kind of analysis document.
    Analysis,

    /// The document is an abstract of a record or document.
    Abstract,

    /// The document is a transcription of a record or document.
    Transcription,

    /// The document is a translation of a record or document.
    Translation,
    Custom(Uri),
}

impl_enumasstring_yaserialize_yadeserialize!(DocumentType, "DocumentType");

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

impl Default for DocumentType {
    fn default() -> Self {
        Self::Custom(Uri::default())
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

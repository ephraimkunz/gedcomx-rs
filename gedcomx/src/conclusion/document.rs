use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, EnumAsString, Id, Lang, Note, ResourceReference, SourceReference,
    Uri,
};

/// The base conceptual model for genealogical data that are managed as textual
/// documents.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Document {
    /// An identifier for the conclusion data. The id is to be used as a "fragment identifier" as defined by [RFC 3986, Section 3.5](https://tools.ietf.org/html/rfc3986#section-3.5).
    #[yaserde(attribute)]
    pub id: Option<Id>,

    /// The locale identifier for the conclusion.
    #[yaserde(attribute, prefix = "xml")]
    pub lang: Option<Lang>,

    /// The list of references to the sources of related to this conclusion.
    /// Note that the sources referenced from conclusions are also considered
    /// to be sources of the entities that contain them. For example, a source
    /// associated with the `Name` of a `Person` is also source for the
    /// `Person`.
    #[yaserde(rename = "source", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    /// A reference to the analysis document explaining the analysis that went
    /// into this conclusion. If provided, MUST resolve to an instance of
    /// [Document](crate::Document) of type
    /// [Analysis](crate::DocumentType::Analysis).
    // TODO: Validate this at compile time somehow?
    #[yaserde(prefix = "gx")]
    pub analysis: Option<ResourceReference>,

    /// A list of notes about this conclusion.
    #[yaserde(rename = "note", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub notes: Vec<Note>,

    /// The level of confidence the contributor has about the data.
    #[yaserde(attribute)]
    pub confidence: Option<ConfidenceLevel>,

    /// The attribution of this conclusion.
    /// If not provided, the attribution of the containing data set (e.g. file)
    /// of the conclusion is assumed.
    #[yaserde(prefix = "gx")]
    pub attribution: Option<Attribution>,

    /// Enumerated value identifying the type of the document.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub document_type: Option<DocumentType>,

    /// Whether this document is to be constrained as an *xtracted conclusion,
    /// meaning it captures information extracted from a single source.
    #[yaserde(attribute)]
    pub extracted: Option<bool>,

    /// The type of text in the `text` property.
    ///
    /// If provided, the value MUST be a [valid text type](https://github.com/FamilySearch/gedcomx/blob/master/specifications/conceptual-model-specification.md#text-types). If no value is provided, "plain" is assumed
    // TODO: Newtype for this?
    #[yaserde(rename = "textType", attribute)]
    pub text_type: Option<String>,

    /// The text of the document.
    #[yaserde(prefix = "gx")]
    pub text: String,
}

impl Document {
    pub fn new(
        id: Option<Id>,
        lang: Option<Lang>,
        sources: Vec<SourceReference>,
        analysis: Option<ResourceReference>,
        notes: Vec<Note>,
        confidence: Option<ConfidenceLevel>,
        attribution: Option<Attribution>,
        document_type: Option<DocumentType>,
        extracted: Option<bool>,
        text_type: Option<String>,
        text: String,
    ) -> Self {
        Self {
            id,
            lang,
            sources,
            analysis,
            notes,
            confidence,
            attribution,
            document_type,
            extracted,
            text_type,
            text,
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
            self.0.id.clone(),
            self.0.lang.clone(),
            self.0.sources.clone(),
            self.0.analysis.clone(),
            self.0.notes.clone(),
            self.0.confidence.clone(),
            self.0.attribution.clone(),
            self.0.document_type.clone(),
            self.0.extracted,
            self.0.text_type.clone(),
            self.0.text.clone(),
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

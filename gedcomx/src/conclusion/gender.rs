use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, EnumAsString, Id, Lang, Note, ResourceReference, SourceReference,
    Uri,
};

/// A gender of a person.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct Gender {
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

    /// The type of the gender.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub gender_type: GenderType,
}

impl Gender {
    pub fn new(
        id: Option<Id>,
        lang: Option<Lang>,
        sources: Vec<SourceReference>,
        analysis: Option<ResourceReference>,
        notes: Vec<Note>,
        confidence: Option<ConfidenceLevel>,
        attribution: Option<Attribution>,
        gender_type: GenderType,
    ) -> Self {
        Self {
            id,
            lang,
            sources,
            analysis,
            notes,
            confidence,
            attribution,
            gender_type,
        }
    }

    pub fn builder(gender_type: GenderType) -> GenderBuilder {
        GenderBuilder::new(gender_type)
    }
}

pub struct GenderBuilder(Gender);

impl GenderBuilder {
    conclusion_builder_functions!(Gender);

    pub(crate) fn new(gender_type: GenderType) -> Self {
        Self(Gender {
            gender_type,
            ..Gender::default()
        })
    }

    pub fn build(&self) -> Gender {
        Gender::new(
            self.0.id.clone(),
            self.0.lang.clone(),
            self.0.sources.clone(),
            self.0.analysis.clone(),
            self.0.notes.clone(),
            self.0.confidence.clone(),
            self.0.attribution.clone(),
            self.0.gender_type.clone(),
        )
    }
}

impl From<GenderType> for Gender {
    fn from(gender_type: GenderType) -> Self {
        Self {
            gender_type,
            ..Self::default()
        }
    }
}

/// Type of gender.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum GenderType {
    /// Male gender.
    Male,

    /// Female gender.
    Female,

    /// Unknown gender.
    Unknown,

    /// Intersex (assignment at birth).
    Intersex,

    Custom(Uri),
}

impl Default for GenderType {
    fn default() -> Self {
        Self::Custom(Uri::from(String::default()))
    }
}

impl_enumasstring_yaserialize_yadeserialize!(GenderType, "GenderType");

impl From<EnumAsString> for GenderType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Male" => Self::Male,
            "http://gedcomx.org/Female" => Self::Female,
            "http://gedcomx.org/Unknown" => Self::Unknown,
            "http://gedcomx.org/Intersex" => Self::Intersex,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for GenderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Male => write!(f, "http://gedcomx.org/Male"),
            Self::Female => write!(f, "http://gedcomx.org/Female"),
            Self::Unknown => write!(f, "http://gedcomx.org/Unknown"),
            Self::Intersex => write!(f, "http://gedcomx.org/Intersex"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::TestData;

    #[test]
    fn json_deserialize() {
        let data = TestData::new();

        let json = r#"{
            "type" : "http://gedcomx.org/Male",
            "id" : "local_id",
            "lang" : "en",
            "sources" : [ {
                "description" : "SD-1",
                "descriptionId" : "Description id of the target source",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                },
                "qualifiers" : [ { "name" : "http://gedcomx.org/RectangleRegion", "value" : "rectangle region value" } ]          
            }],
            "analysis" : {
              "resource" : "http://identifier/for/analysis/document"
            },
            "notes" : [ {
                "lang" : "en",
                "subject" : "subject",
                "text" : "This is a note",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                }        
            } ],
            "confidence" : "http://gedcomx.org/High",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            }  
        }"#;

        let gender: Gender = serde_json::from_str(json).unwrap();

        assert_eq!(
            gender,
            Gender {
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
                gender_type: GenderType::Male,
            }
        )
    }

    #[test]
    fn xml_deserialize() {
        let xml = "<Gender xmlns=\"http://gedcomx.org/v1/\" type=\"http://gedcomx.org/Male\" />";

        let gender: Gender = yaserde::de::from_str(xml).unwrap();

        assert_eq!(gender, Gender::builder(GenderType::Male).build());
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let gender = Gender {
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
            gender_type: GenderType::Male,
        };

        let json = serde_json::to_string(&gender).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"type":"http://gedcomx.org/Male"}"#
        )
    }

    #[test]
    fn xml_serialize() {
        let gender = Gender::builder(GenderType::Male).build();

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };
        let xml = yaserde::ser::to_string_with_config(&gender, &config).unwrap();

        assert_eq!(
            xml,
            "<Gender xmlns=\"http://gedcomx.org/v1/\" type=\"http://gedcomx.org/Male\" />"
        );
    }
}

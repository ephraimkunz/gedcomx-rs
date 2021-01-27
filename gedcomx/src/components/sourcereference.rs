use super::EnumAsString;
use crate::{
    components::{Attribution, Id, Qualifier, Uri},
    GedcomxError, Result, SourceDescription,
};
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SourceReference {
    pub description: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_id: Option<Id>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub qualifiers: Vec<Qualifier>,
}

impl SourceReference {
    pub fn new(
        description: Uri,
        description_id: Option<Id>,

        attribution: Option<Attribution>,

        qualifiers: Vec<Qualifier>,
    ) -> Self {
        Self {
            description,
            description_id,
            attribution,
            qualifiers,
        }
    }

    pub fn builder_with_raw(description: Uri) -> SourceReferenceBuilder {
        SourceReferenceBuilder::new(description)
    }

    /// # Errors
    ///
    /// Will return `GedcomxError` if a conversion into `SourceReference` fails.
    /// This happens if the argument we are converting has no Id set.
    pub fn builder(description: &SourceDescription) -> Result<SourceReferenceBuilder> {
        Ok(SourceReferenceBuilder::new(description.try_into()?))
    }
}

pub struct SourceReferenceBuilder(SourceReference);

impl SourceReferenceBuilder {
    pub(crate) fn new(description: Uri) -> Self {
        Self(SourceReference {
            description,
            ..SourceReference::default()
        })
    }

    pub fn description_id<I: Into<String>>(&mut self, id: I) -> &mut Self {
        self.0.description_id = Some(id.into());
        self
    }

    pub fn build(&self) -> SourceReference {
        SourceReference::new(
            self.0.description.clone(),
            self.0.description_id.clone(),
            self.0.attribution.clone(),
            self.0.qualifiers.clone(),
        )
    }
}

impl TryFrom<&SourceDescription> for SourceReference {
    type Error = GedcomxError;
    fn try_from(s: &SourceDescription) -> std::result::Result<Self, Self::Error> {
        match &s.id {
            Some(id) => Ok(Self::builder_with_raw(format!("#{}", id).into()).build()),
            None => Err(GedcomxError::NoId("SourceDescription".to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum SourceReferenceQualifier {
    CharacterRegion,
    RectangleRegion,
    TimeRegion,
    Custom(Uri),
}

impl From<EnumAsString> for SourceReferenceQualifier {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/CharacterRegion" => Self::CharacterRegion,
            "http://gedcomx.org/RectangleRegion" => Self::RectangleRegion,
            "http://gedcomx.org/TimeRegion" => Self::TimeRegion,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for SourceReferenceQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::CharacterRegion => write!(f, "http://gedcomx.org/CharacterRegion"),
            Self::RectangleRegion => write!(f, "http://gedcomx.org/RectangleRegion"),
            Self::TimeRegion => write!(f, "http://gedcomx.org/TimeRegion"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::TestData;

    #[test]
    fn json_deserialize() {
        let data = TestData::new();

        let json = r#"{
            "description" : "SD-1",
            "descriptionId" : "Description id of the target source",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            },
            "qualifiers" : [ { "name" : "http://gedcomx.org/RectangleRegion", "value" : "rectangle region value" } ]          
        }"#;

        let source_reference: SourceReference = serde_json::from_str(json).unwrap();

        assert_eq!(source_reference, data.source_reference)
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "description" : "SD-1"
        }"#;

        let source_reference: SourceReference = serde_json::from_str(json).unwrap();
        assert_eq!(
            source_reference,
            SourceReference::builder_with_raw(Uri::from("SD-1")).build()
        )
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let source_reference = data.source_reference;

        let json = serde_json::to_string(&source_reference).unwrap();
        assert_eq!(
            json,
            r#"{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}"#
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let source_reference = SourceReference::builder_with_raw(Uri::from("SD-1")).build();

        let json = serde_json::to_string(&source_reference).unwrap();
        assert_eq!(json, r#"{"description":"SD-1"}"#);
    }
}

use std::{
    convert::{TryFrom, TryInto},
    fmt,
};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Attribution, EnumAsString, GedcomxError, Id, Qualifier, Result, SourceDescription, Uri,
};

/// A reference to a source description.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SourceReference {
    /// Reference to a description of the target source.
    ///
    /// MUST resolve to an instance of http://gedcomx.org/v1/SourceDescription.
    // TODO: Enforce
    pub description: Uri,

    /// The id of the target source.
    pub description_id: Option<Id>,

    /// The attribution of this source reference.
    ///
    /// If not provided, the attribution of the containing resource of the
    /// source reference is assumed.
    pub attribution: Option<Attribution>,

    /// Qualifiers for the reference, used to identify specific fragments of the
    /// source that are being referenced.
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
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`SourceReference`](crate::SourceReference) fails.
    /// This happens if `description` has no `id` set.
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

    pub fn description_id<I: Into<Id>>(&mut self, id: I) -> &mut Self {
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
            Some(id) => Ok(Self::builder_with_raw(id.into()).build()),
            None => Err(GedcomxError::NoId("SourceDescription".to_string())),
        }
    }
}

/// Source reference qualifiers.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum SourceReferenceQualifier {
    /// A region of text in a digital document, in the form of a,b where a is
    /// the index of the start character and b is the index of the end
    /// character. The meaning of this qualifier is undefined if the source
    /// being referenced is not a digital document.
    CharacterRegion,

    /// A rectangular region of a digital image. The value of the qualifier is
    /// interpreted as a series of four comma-separated numbers. If all of the
    /// numbers is less than 1, the value is interpreted in the form of
    /// x1,y1,x2,y2 where x1,y1 is the relative percentage-based coordinates of
    /// the top-left corner of the rectangle and x2,y2 is the relative
    /// percentage-based coordinates of the bottom-right corner of the
    /// rectangle. If any of the numbers is more than 1, the value is
    /// interpreted in the form of x,y,w,h where x is the point on the X axis of
    /// the image in pixels, y is the point on the Y axis in pixels, w is the
    /// width of the rectangle in pixels, and h in the height of the rectangle
    /// in pixels.
    RectangleRegion,

    /// A region of time of a digital audio or video recording, in the form of
    /// a,b where a is the starting point in milliseconds and b is the ending
    /// point in milliseconds. The meaning of this qualifier is undefined if the
    /// source being referenced is not a digital audio or video recording.
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
    use crate::TestData;

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

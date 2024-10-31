use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Attribution, GedcomxError, Id, Qualifier, Result, SourceDescription, Uri};

/// A reference to a source description.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SourceReference {
    /// Reference to a description of the target source.
    ///
    /// MUST resolve to an instance of http://gedcomx.org/v1/SourceDescription.
    #[yaserde(attribute)]
    pub description: Uri,

    /// The id of the target source.
    #[yaserde(rename = "descriptionId", attribute)]
    pub description_id: Option<Id>,

    /// The attribution of this source reference.
    ///
    /// If not provided, the attribution of the containing resource of the
    /// source reference is assumed.
    #[yaserde(prefix = "gx")]
    pub attribution: Option<Attribution>,

    /// Qualifiers for the reference, used to identify specific fragments of the
    /// source that are being referenced.
    ///
    /// If present, use of a
    /// [`SourceReferenceQualifier`](crate::SourceReferenceQualifier) is
    /// RECOMMENDED.
    #[yaserde(rename = "qualifier", prefix = "gx")]
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

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`SourceReference`](crate::SourceReference) fails.
    /// This happens if `description` has no `id` set.
    pub fn builder(description: &SourceDescription) -> Result<SourceReferenceBuilder> {
        Ok(SourceReferenceBuilder::new(description.try_into()?))
    }
}

impl Arbitrary for SourceReference {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(
            Uri::arbitrary(g),
            Some(Id::arbitrary(g)),
            Some(Attribution::arbitrary(g)),
            vec![Qualifier::arbitrary(g)],
        )
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

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`SourceReference`](crate::SourceReference) fails.
    /// This happens if `description` has no `id` set.
    pub fn description(&mut self, description: &SourceDescription) -> Result<&mut Self> {
        self.0.description = description.try_into()?;
        Ok(self)
    }

    pub fn description_id<I: Into<Id>>(&mut self, id: I) -> &mut Self {
        self.0.description_id = Some(id.into());
        self
    }

    pub fn attribution(&mut self, attribution: Attribution) -> &mut Self {
        self.0.attribution = Some(attribution);
        self
    }

    pub fn qualifier(&mut self, qualifier: Qualifier) -> &mut Self {
        self.0.qualifiers.push(qualifier);
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
        s.id.as_ref().map_or_else(
            || Err(GedcomxError::no_id_error(&s)),
            |id| Ok(Self::new(id.into(), None, None, vec![])),
        )
    }
}

/// Source reference qualifiers.
#[derive(Debug, PartialEq, Clone, Eq)]
#[non_exhaustive]
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
}

impl FromStr for SourceReferenceQualifier {
    type Err = GedcomxError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "http://gedcomx.org/CharacterRegion" => Ok(Self::CharacterRegion),
            "http://gedcomx.org/RectangleRegion" => Ok(Self::RectangleRegion),
            "http://gedcomx.org/TimeRegion" => Ok(Self::TimeRegion),
            _ => Err(GedcomxError::QualifierParse {
                parsed_string: s.to_string(),
            }),
        }
    }
}

impl fmt::Display for SourceReferenceQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::CharacterRegion => write!(f, "http://gedcomx.org/CharacterRegion"),
            Self::RectangleRegion => write!(f, "http://gedcomx.org/RectangleRegion"),
            Self::TimeRegion => write!(f, "http://gedcomx.org/TimeRegion"),
        }
    }
}

impl Arbitrary for SourceReferenceQualifier {
    fn arbitrary(g: &mut Gen) -> Self {
        let options = vec![
            Self::CharacterRegion,
            Self::RectangleRegion,
            Self::TimeRegion,
        ];

        g.choose(&options).unwrap().clone()
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

        assert_eq!(source_reference, data.source_reference);
    }

    #[test]
    fn xml_deserialize() {
        let data = TestData::new();

        let xml = r#"
        <SourceReference xmlns="http://gedcomx.org/v1/" description="SD-1" descriptionId="Description id of the target source">
        <attribution>
          <contributor resource="A-1" />
          <modified>2014-03-07T07:00:00</modified>
        </attribution>
        <qualifier name="http://gedcomx.org/RectangleRegion">rectangle region value</qualifier>    
      </SourceReference>
      "#;

        let source_reference: SourceReference = yaserde::de::from_str(xml).unwrap();

        assert_eq!(source_reference, data.source_reference);
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "description" : "SD-1"
        }"#;

        let source_reference: SourceReference = serde_json::from_str(json).unwrap();
        assert_eq!(
            source_reference,
            SourceReference::new(Uri::from("SD-1"), None, None, vec![])
        );
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
    fn xml_serialize() {
        let data = TestData::new();

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };
        let xml = yaserde::ser::to_string_with_config(&data.source_reference, &config).unwrap();

        let expected_xml = r#"<SourceReference xmlns="http://gedcomx.org/v1/" description="SD-1" descriptionId="Description id of the target source"><attribution><contributor resource="A-1" /><modified>2014-03-07T07:00:00Z</modified></attribution><qualifier name="http://gedcomx.org/RectangleRegion">rectangle region value</qualifier></SourceReference>"#;

        assert_eq!(xml, expected_xml);
    }

    #[test]
    fn json_serialize_optional_fields() {
        let source_reference = SourceReference::new(Uri::from("SD-1"), None, None, vec![]);

        let json = serde_json::to_string(&source_reference).unwrap();
        assert_eq!(json, r#"{"description":"SD-1"}"#);
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: SourceReference) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: SourceReference = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: SourceReference) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: SourceReference = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

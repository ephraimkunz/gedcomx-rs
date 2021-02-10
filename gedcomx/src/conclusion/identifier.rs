use std::fmt;

use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{EnumAsString, Uri};

/// An identifier of a genealogical resource.
// I think this will need custom JSON serialization / deserialization. Needs to be a map of
// identifier_type -> [uri].
///
/// # Examples
/// An instance of Person with an identifier of type
/// [`Primary`](crate::IdentifierType::Primary) and value "12345" is merged into
/// an instance of `Person` with an identifier of type
/// [`Primary`](crate::IdentifierType::Primary) and value "67890". `Person`
/// "67890" assumes an identifier of type
/// [`Deprecated`](crate::IdentifierType::Deprecated) and value "12345". The
/// identifier type [`Deprecated`](crate::IdentifierType::Deprecated) is used
/// because the merged person "12345" now has identifier of type
/// [`Primary`](crate::IdentifierType::Primary)with value "67890".
///
/// A description of Salt Lake City, Utah, United States is provided using an
/// instance of [`PlaceDescription`](crate::PlaceDescription). Salt Lake City is
/// maintained in the Geographic Names Information System (GNIS), an external
/// place authority. The description of Salt Lake City might identify the associated GNIS resource using an identifier of type [`Authority`](crate::IdentifierType::Authority) with value "<http://geonames.usgs.gov/pls/gnispublic/f?p=gnispq:3:::NO::P3_FID:2411771>".
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Identifier {
    /// The value of the identifier.
    pub value: Uri,

    /// Identifies how the identifier is to be used and the nature of the
    /// resource to which the identifier resolves.
    ///
    /// If no type is provided, the usage and nature of the identifier is
    /// application-specific.
    #[serde(rename = "type")]
    pub identifier_type: Option<IdentifierType>,
}

impl Identifier {
    pub fn new<I: Into<Uri>>(value: I, identifier_type: Option<IdentifierType>) -> Self {
        Self {
            value: value.into(),
            identifier_type,
        }
    }
}

impl yaserde::YaSerialize for Identifier {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let mut start_builder = xml::writer::XmlEvent::start_element("identifier");

        let identifier_type_value;
        if let Some(t) = &self.identifier_type {
            identifier_type_value = t.to_string();
            start_builder = start_builder.attr("type", &identifier_type_value); // TODO: Should I use the pattern from https://docs.rs/xml-rs/0.8.3/xml/writer/events/struct.StartElementBuilder.html to avoid all the build() calls for users of this library?
        }

        writer.write(start_builder).map_err(|e| e.to_string())?;
        writer
            .write(xml::writer::XmlEvent::characters(&self.value.to_string()))
            .map_err(|e| e.to_string())?;
        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<xml::attribute::OwnedAttribute>,
        namespace: xml::namespace::Namespace,
    ) -> Result<
        (
            Vec<xml::attribute::OwnedAttribute>,
            xml::namespace::Namespace,
        ),
        String,
    > {
        Ok((attributes, namespace))
    }
}

impl yaserde::YaDeserialize for Identifier {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        let mut identifier_type = None;

        if let xml::reader::XmlEvent::StartElement {
            name: element_name,
            attributes,
            ..
        } = reader.next_event()?
        {
            let expected_name = "identifier".to_owned();
            if element_name.local_name != expected_name {
                return Err(format!(
                    "Wrong StartElement name: {}, expected: {}",
                    element_name, expected_name
                ));
            }

            if attributes.len() > 1 {
                return Err(format!(
                    "Too many attributes: {:?}, expected 0 or 1",
                    attributes
                ));
            }

            if let Some(t) = attributes.first() {
                let expected_name = "type";
                if t.name.local_name != "type" {
                    return Err(format!(
                        "Wrong attribute name: {}, expected: {}",
                        t.name, expected_name
                    ));
                }

                identifier_type = Some(IdentifierType::from(EnumAsString(t.value.clone())));
            }
        } else {
            return Err("StartElement missing".to_string());
        }

        if let xml::reader::XmlEvent::Characters(text) = reader.next_event()? {
            Ok(Self {
                value: text.into(),
                identifier_type,
            })
        } else {
            Err("Uri missing".to_string())
        }
    }
}

/// Standard identifier types.
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum IdentifierType {
    /// The primary identifier for the resource.
    ///
    /// The value of the identifier MUST resolve to the instance of Subject to
    /// which the identifier applies.
    Primary,

    /// An identifier for the resource in an external authority or other expert
    /// system.
    ///
    /// The value of the identifier MUST resolve to a public, authoritative,
    /// source for information about the Subject to which the identifier
    /// applies.
    Authority,

    /// An identifier that has been relegated, deprecated, or otherwise
    /// downgraded.
    ///
    /// This identifier is commonly used as the result of a merge when what was
    /// once a primary identifier for a resource is no longer the primary
    /// identifier. The value of the identifier MUST resolve to the instance
    /// of Subject to which the identifier applies.
    Deprecated,

    Custom(Uri),
}

impl Default for IdentifierType {
    fn default() -> Self {
        Self::Custom(Uri::default())
    }
}

impl From<EnumAsString> for IdentifierType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Primary" => Self::Primary,
            "http://gedcomx.org/Authority" => Self::Authority,
            "http://gedcomx.org/Deprecated" => Self::Deprecated,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for IdentifierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Primary => write!(f, "http://gedcomx.org/Primary"),
            Self::Authority => write!(f, "http://gedcomx.org/Authority"),
            Self::Deprecated => write!(f, "http://gedcomx.org/Deprecated"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

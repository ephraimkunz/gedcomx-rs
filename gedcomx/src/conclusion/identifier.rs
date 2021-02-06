use crate::{EnumAsString, Uri};
use serde::{Deserialize, Serialize};
use std::fmt;
use yaserde_derive::{YaDeserialize, YaSerialize};

// I think this will need custom JSON serialization / deserialization. Needs to be a map of typee -> [uri].
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Identifier {
    pub uri: Uri,

    #[serde(rename = "type")]
    pub identifier_type: Option<IdentifierType>,
}

impl Identifier {
    pub fn new<I: Into<Uri>>(uri: I, identifier_type: Option<IdentifierType>) -> Self {
        Self {
            uri: uri.into(),
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
            .write(xml::writer::XmlEvent::characters(&self.uri.to_string()))
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
                uri: text.into(),
                identifier_type,
            })
        } else {
            Err("Uri missing".to_string())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum IdentifierType {
    Primary,
    Authority,
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

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{EnumAsString, Uri};

/// An identifier of a genealogical resource.
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
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Identifier {
    /// The value of the identifier.
    pub value: Uri,

    /// Identifies how the identifier is to be used and the nature of the
    /// resource to which the identifier resolves.
    ///
    /// If no type is provided, the usage and nature of the identifier is
    /// application-specific.
    pub identifier_type: Option<IdentifierType>,

    // Private, to properly round-trip JSON.
    value_in_vec: bool,
}

impl Identifier {
    pub fn new<I: Into<Uri>>(value: I, identifier_type: Option<IdentifierType>) -> Self {
        Self {
            value: value.into(),
            identifier_type,
            value_in_vec: true,
        }
    }
}

impl yaserde::YaSerialize for Identifier {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let provided_start_event_name = writer.get_start_event_name();
        let start_name = provided_start_event_name.as_deref().unwrap_or("identifier");
        let mut start_builder = xml::writer::XmlEvent::start_element(start_name);

        let identifier_type_value;
        if let Some(t) = &self.identifier_type {
            identifier_type_value = t.to_string();
            start_builder = start_builder.attr("type", &identifier_type_value);
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
                value_in_vec: true,
            })
        } else {
            Err("Uri missing".to_string())
        }
    }
}

pub(in crate) mod serde_vec_identifier_to_map {
    use std::{collections::HashMap, fmt};

    use serde::{
        de::{Deserializer, MapAccess, Visitor},
        ser::{SerializeMap, Serializer},
        Deserialize, Serialize,
    };

    use crate::{EnumAsString, Identifier, Uri};

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum VecOrUri {
        Vec(Vec<Uri>),
        Uri(Uri),
    }

    /// # Errors
    /// Returns serde errors if serialization fails.
    pub fn serialize<S>(identifiers: &[Identifier], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hashmap: HashMap<String, VecOrUri> = HashMap::with_capacity(identifiers.len());
        for id in identifiers {
            let e = hashmap
                .entry(
                    id.identifier_type
                        .as_ref()
                        .map_or(String::from("$"), std::string::ToString::to_string),
                )
                .or_insert_with(|| {
                    if id.value_in_vec {
                        VecOrUri::Vec(Vec::new())
                    } else {
                        VecOrUri::Uri(Uri::default())
                    }
                });

            match e {
                VecOrUri::Uri(u) => *u = id.value.clone(),
                VecOrUri::Vec(v) => {
                    v.push(id.value.clone());
                    v.sort_by_key(std::string::ToString::to_string);
                }
            }
        }

        let mut map = serializer.serialize_map(Some(identifiers.len()))?;

        let mut keys = hashmap.keys().collect::<Vec<_>>();
        keys.sort();

        for k in keys {
            if let Some(v) = hashmap.get(k) {
                map.serialize_entry(k, v)?;
            }
        }

        map.end()
    }

    struct IdentifierVisitor;

    impl<'de> Visitor<'de> for IdentifierVisitor {
        // The type that our Visitor is going to produce.
        type Value = Vec<Identifier>;

        // Format a message stating what data this Visitor expects to receive.
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("A map of identifier types to lists of values")
        }

        // Deserialize Value from an abstract "map" provided by the
        // Deserializer. The MapAccess input is a callback provided by
        // the Deserializer to let us see each entry in the map.
        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut identifiers = Vec::with_capacity(access.size_hint().unwrap_or(0));

            // While there are entries remaining in the input, add them
            // into our vec. According to https://github.com/FamilySearch/gedcomx/blob/master/specifications/json-format-specification.md#31-the-identifier-data-type,
            // identifiers values may be either a single string or a vec.

            while let Ok(Some((key, value))) = access.next_entry::<EnumAsString, VecOrUri>() {
                match value {
                    VecOrUri::Vec(vec) => {
                        for v in vec {
                            let k = if key.0 == "$" {
                                None
                            } else {
                                Some(key.clone().into())
                            };
                            identifiers.push(Identifier::new(v, k));
                        }
                    }
                    VecOrUri::Uri(uri) => {
                        let k = if key.0 == "$" {
                            None
                        } else {
                            Some(key.clone().into())
                        };
                        let mut identifier = Identifier::new(uri, k);
                        identifier.value_in_vec = false;
                        identifiers.push(identifier);
                    }
                }
            }

            Ok(identifiers)
        }
    }

    /// # Errors
    /// Returns serde errors if deserialization fails.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Identifier>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_map(IdentifierVisitor {})
    }
}

/// Standard identifier types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl_enumasstring_yaserialize_yadeserialize!(IdentifierType, "IdentifierType");

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

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};
    use yaserde::ser::Config;

    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestIdentifierGroup {
        #[serde(with = "crate::serde_vec_identifier_to_map", flatten)]
        identifiers: Vec<Identifier>,
    }

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "$":["untyped_identifier1","untyped_identifier2"],
            "http://custom.org/SingleValuedIdentifierType":["singlevalued1"],
            "http://gedcomx.org/Primary":["primary1","primary2"],
            "http://custom.org/SingleValuedIdentifierType2": "nolist"
        }"#;

        let mut no_list_identifier = Identifier::new(
            "nolist",
            Some(IdentifierType::Custom(
                "http://custom.org/SingleValuedIdentifierType2".into(),
            )),
        );
        no_list_identifier.value_in_vec = false;

        let identifiers: TestIdentifierGroup = serde_json::from_str(&json).unwrap();

        let expected_identifiers = TestIdentifierGroup {
            identifiers: vec![
                Identifier::new("untyped_identifier1", None),
                Identifier::new("untyped_identifier2", None),
                Identifier::new(
                    "singlevalued1",
                    Some(IdentifierType::Custom(
                        "http://custom.org/SingleValuedIdentifierType".into(),
                    )),
                ),
                Identifier::new("primary1", Some(IdentifierType::Primary)),
                Identifier::new("primary2", Some(IdentifierType::Primary)),
                no_list_identifier,
            ],
        };

        assert_eq!(identifiers, expected_identifiers)
    }

    #[test]
    fn json_serialize() {
        let identifiers = TestIdentifierGroup {
            identifiers: vec![
                Identifier::new("untyped_identifier1", None),
                Identifier::new("untyped_identifier2", None),
                Identifier::new("primary1", Some(IdentifierType::Primary)),
                Identifier::new("primary2", Some(IdentifierType::Primary)),
                Identifier::new(
                    "singlevalued1",
                    Some(IdentifierType::Custom(
                        "http://custom.org/SingleValuedIdentifierType".into(),
                    )),
                ),
            ],
        };

        let json = serde_json::to_string(&identifiers).unwrap();
        let expected_json = r#"{"$":["untyped_identifier1","untyped_identifier2"],"http://custom.org/SingleValuedIdentifierType":["singlevalued1"],"http://gedcomx.org/Primary":["primary1","primary2"]}"#;

        assert_eq!(json, expected_json)
    }

    #[test]
    fn xml_deserialize() {
        let xml = r#"<identifier type="http://gedcomx.org/Primary">https://familysearch.org/platform/records/collections</identifier>"#;

        let expected_identifier = Identifier::new(
            "https://familysearch.org/platform/records/collections",
            Some(IdentifierType::Primary),
        );

        assert_eq!(
            expected_identifier,
            yaserde::de::from_str::<Identifier>(&xml).unwrap()
        );
    }

    #[test]
    fn xml_serialize() {
        let identifier = Identifier::new(
            "https://familysearch.org/platform/records/collections",
            Some(IdentifierType::Primary),
        );
        let xml = yaserde::ser::to_string_with_config(
            &identifier,
            &Config {
                write_document_declaration: false,
                ..Config::default()
            },
        )
        .unwrap();
        let expected_xml = r#"<identifier type="http://gedcomx.org/Primary">https://familysearch.org/platform/records/collections</identifier>"#;

        assert_eq!(xml, expected_xml)
    }
}

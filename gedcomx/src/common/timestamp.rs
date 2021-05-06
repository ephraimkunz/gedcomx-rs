use std::{fmt, str::FromStr};

use chrono::{serde::ts_milliseconds, DateTime, NaiveDateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};
use yaserde::{YaDeserialize, YaSerialize};

/// When an event something was created or modified.
///
/// Not the same as [`Date`](crate::Date) which represents things in the Gedcomx
/// date format.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Timestamp {
    #[serde(with = "ts_milliseconds")]
    value: DateTime<Utc>,

    //http://books.xmlschemata.org/relaxng/ch19-77049.html. XML dateTime allows there to be no timezone on a time, which means it's "undetermined".
    // However the JSON representation is as a timestamp that assumes UTC. So in order to correctly
    // roundtrip this timezone when parsing XML, we'll store whether it is undetermined.
    // However there will be no way for the user to set this and any interaction they have with
    // this struct will be through DateTime<UTC>.
    #[serde(skip)]
    undetermined_tz: bool,
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl YaSerialize for Timestamp {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        if let Some(start_event_name) = writer.get_start_event_name() {
            writer
                .write(xml::writer::XmlEvent::start_element(
                    start_event_name.as_str(),
                ))
                .map_err(|e| e.to_string())?;
        }

        writer
            .write(xml::writer::XmlEvent::characters(&self.to_string()))
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

impl YaDeserialize for Timestamp {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        if let xml::reader::XmlEvent::StartElement { .. } = reader.next_event()? {
        } else {
            return Err("No start event".to_string());
        }

        let timestamp;
        if let xml::reader::XmlEvent::Characters(text) = reader.next_event()? {
            timestamp = text.parse().map_err(|e: ParseError| e.to_string())?;
        } else {
            return Err("Characters missing".to_string());
        }

        if let xml::reader::XmlEvent::EndElement { .. } = reader.next_event()? {
            Ok(timestamp)
        } else {
            Err("No end event".to_string())
        }
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Self {
            value: dt,
            undetermined_tz: false,
        }
    }
}

impl From<NaiveDateTime> for Timestamp {
    fn from(dt: NaiveDateTime) -> Self {
        Self {
            value: DateTime::from_utc(dt, Utc),
            undetermined_tz: true,
        }
    }
}

// From XML's xsd:dateTime string to Timestamp. Logic from https://github.com/lumeohq/xsd-parser-rs/blob/main/xsd-types/src/types/datetime.rs

impl Default for Timestamp {
    fn default() -> Self {
        Self {
            value: DateTime::parse_from_rfc3339("0001-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            undetermined_tz: false,
        }
    }
}

impl FromStr for Timestamp {
    type Err = ParseError;

    // Note:
    // `parse_from_rfc3339` parses an RFC 3339 and ISO 8601 date and time string.
    // XSD follows ISO 8601, which allows no time zone at the end of literal.
    // Since RFC 3339 does not allow such behavior, the function tries to add
    // 'Z' (which equals "+00:00") in case there is no timezone provided.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tz_provided = s.ends_with('Z') || s.contains('+') || s.matches('-').count() == 3;
        let s_with_timezone = if tz_provided {
            s.to_string()
        } else {
            format!("{}Z", s)
        };

        match DateTime::parse_from_rfc3339(&s_with_timezone) {
            Ok(dt) => Ok(Self {
                value: dt.with_timezone(&Utc),
                undetermined_tz: !tz_provided,
            }),
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // to_rfc3339 always includes a timezone. Since XSD follows ISO 8601, timezones
        // can be unspecified. If we know this Timestamp has an unspecified
        // timezone, remove it from the string.
        let full = self
            .value
            .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true);
        let partial = if self.undetermined_tz {
            &full[..19]
        } else {
            full.as_str()
        };

        write!(f, "{}", partial)
    }
}

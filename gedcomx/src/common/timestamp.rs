use std::{fmt, str::FromStr};

use chrono::{DateTime, NaiveDateTime, ParseError, TimeZone, Utc, serde::ts_milliseconds};
use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use yaserde::{YaDeserialize, YaSerialize};

/// When an event something was created or modified.
///
/// Not the same as [`Date`](crate::Date) which represents things in the Gedcomx
/// date format.
///
/// In JSON this is represented as the number of milliseconds since the Unix
/// epoch. In XML it's represented by xsd:dateTime.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
#[non_exhaustive]
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

// Don't consider undetermined_tz when comparing, since that's just to ensure
// proper XML roundtripping.
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

        let timestamp = if let xml::reader::XmlEvent::Characters(text) = reader.next_event()? {
            text.parse().map_err(|e: ParseError| e.to_string())?
        } else {
            return Err("Characters missing".to_string());
        };

        // Yaserde seems to depend on us not consuming the end event.
        if let xml::reader::XmlEvent::EndElement { .. } = reader.peek()? {
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
            value: dt.and_utc(),
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
            format!("{s}Z")
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

        write!(f, "{partial}")
    }
}

impl Arbitrary for Timestamp {
    fn arbitrary(g: &mut Gen) -> Self {
        let naive = chrono::NaiveDate::from_ymd_opt(
            arbitrary_between!(i32; g, 1900, 2200),
            arbitrary_between!(u32; g, 1, 12),
            arbitrary_between!(u32; g, 1, 28),
        )
        .and_then(|date| {
            date.and_hms_opt(
                arbitrary_between!(u32; g, 0, 23),
                arbitrary_between!(u32; g, 0, 59),
                arbitrary_between!(u32; g, 0, 59),
            )
        })
        .expect("Unable to create arbitrary time");

        Utc.from_utc_datetime(&naive).into()
    }
}

#[cfg(test)]
mod test {
    use chrono::{FixedOffset, NaiveDate, TimeZone};

    use super::*;

    #[test]
    fn json_deserialize() {
        let json = "1338494969";
        let chrono::LocalResult::Single(timestamp) = Utc.timestamp_millis_opt(1_338_494_969) else {
            panic!("Invalid timestamp")
        };

        let expected = Timestamp::from(timestamp);
        assert_eq!(serde_json::from_str::<Timestamp>(json).unwrap(), expected);
    }

    #[test]
    fn json_serialize() {
        let timestamp = Timestamp::from(match Utc.timestamp_millis_opt(1_338_494_969) {
            chrono::LocalResult::Single(t) => t,
            _ => panic!("Invalid timestamp"),
        });
        let expected = "1338494969";
        assert_eq!(serde_json::to_string(&timestamp).unwrap(), expected);
    }

    #[test]
    fn xml_deserialize() {
        // No timezone.
        let offset = FixedOffset::east_opt(0).expect("Invalid offset");
        let dt = offset
            .with_ymd_and_hms(2020, 3, 7, 4, 40, 0)
            .single()
            .expect("Invalid date");
        assert_eq!(
            Timestamp::from_str("2020-03-07T04:40:00"),
            Ok(Timestamp {
                value: dt.into(),
                undetermined_tz: true
            })
        );

        // Timezone "Z".
        assert_eq!(
            Timestamp::from_str("2020-03-07T04:40:00Z"),
            Ok(Timestamp {
                value: dt.into(),
                undetermined_tz: false
            })
        );

        // Positive offset.
        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).expect("Invalid offset");
        let dt = offset
            .with_ymd_and_hms(2020, 3, 7, 4, 40, 0)
            .single()
            .expect("Invalid date");
        assert_eq!(
            Timestamp::from_str("2020-03-07T04:40:00+06:30"),
            Ok(Timestamp {
                value: dt.into(),
                undetermined_tz: false
            })
        );

        // Negative offset.
        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).expect("Invalid offset");
        let dt = offset
            .with_ymd_and_hms(2020, 3, 7, 4, 40, 0)
            .single()
            .expect("Invalid date");
        assert_eq!(
            Timestamp::from_str("2020-03-07T04:40:00-06:30"),
            Ok(Timestamp {
                value: dt.into(),
                undetermined_tz: false
            })
        );
    }

    #[test]
    fn xml_serialize() {
        // Timezone +00:00.
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .and_then(|d| d.and_hms_opt(4, 40, 0))
            .expect("Invalid date");
        let dt = dt_utc.and_utc();
        assert_eq!(
            Timestamp {
                value: dt,
                undetermined_tz: false
            }
            .to_string(),
            "2020-03-07T04:40:00Z"
        );

        // Positive offset.
        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).expect("Invalid offset");
        let dt = offset
            .with_ymd_and_hms(2020, 3, 7, 4, 40, 0)
            .single()
            .expect("Invalid date");
        assert_eq!(
            Timestamp {
                value: dt.into(),
                undetermined_tz: false
            }
            .to_string(),
            "2020-03-06T22:10:00Z"
        );

        // Negative offset.
        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).expect("Invalid offset");
        let dt = offset
            .with_ymd_and_hms(2020, 3, 7, 4, 40, 0)
            .single()
            .expect("Invalid date");
        assert_eq!(
            Timestamp {
                value: dt.into(),
                undetermined_tz: false
            }
            .to_string(),
            "2020-03-07T11:10:00Z"
        );

        // Undetermined timezone.
        let offset = FixedOffset::west_opt(0).expect("Invalid offset");
        let dt = offset
            .with_ymd_and_hms(2020, 3, 7, 4, 40, 0)
            .single()
            .expect("Invalid date");
        assert_eq!(
            Timestamp {
                value: dt.into(),
                undetermined_tz: true
            }
            .to_string(),
            "2020-03-07T04:40:00"
        );
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: Timestamp) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: Timestamp = serde_json::from_str(&json).unwrap();
        input == from_json
    }
}

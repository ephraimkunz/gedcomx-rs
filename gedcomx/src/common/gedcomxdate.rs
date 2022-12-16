use std::{convert::TryFrom, fmt, fmt::Write, str};

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};

use crate::GedcomxError;

/// Newtype wrapping `GedcomxDate` from the `gedcomx_date` crate and adding the
/// ability to generate a formal string (via the `Display` trait), failably
/// parse from a string (via the `FromStr` trait), and serialize and
/// deseserialize into JSON and XML.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(try_from = "String", into = "String")]
pub struct GedcomxDate(pub gedcomx_date::GedcomxDate);

impl yaserde::YaSerialize for GedcomxDate {
    fn serialize<W: std::io::Write>(
        &self,
        writer: &mut yaserde::ser::Serializer<W>,
    ) -> Result<(), String> {
        let yaserde_label = writer
            .get_start_event_name()
            .unwrap_or_else(|| "GedcomxDate".to_string());
        let struct_start_event = xml::writer::XmlEvent::start_element(yaserde_label.as_ref())
            .default_ns("http://gedcomx.org/v1/");
        let event: xml::writer::events::XmlEvent = struct_start_event.into();
        let _ret = writer.write(event);

        let _ret = writer.write(xml::writer::XmlEvent::characters(self.to_string().as_str()));

        let _ret = writer.write(xml::writer::events::XmlEvent::end_element());

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

impl yaserde::YaDeserialize for GedcomxDate {
    fn deserialize<R: std::io::Read>(
        reader: &mut yaserde::de::Deserializer<R>,
    ) -> Result<Self, String> {
        if let xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()?.clone() {
            let expected_name = "formal".to_owned();
            if name.local_name != expected_name {
                return Err(format!(
                    "Wrong StartElement name: {name}, expected: {expected_name}"
                ));
            }
            let _next = reader.next_event();
        } else {
            return Err("StartElement missing".to_string());
        }

        if let xml::reader::XmlEvent::Characters(text) = reader.peek()?.clone() {
            text.parse::<Self>().map_err(|e| e.to_string())
        } else {
            Err("Characters missing".to_string())
        }
    }
}
impl str::FromStr for GedcomxDate {
    type Err = GedcomxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        gedcomx_date::parse(s)
            .map(Self)
            .map_err(|e| GedcomxError::DateParse {
                parsed_string: s.to_string(),
                error: e,
            })
    }
}

// TryFrom and From<> impls are so we can have Serde auto-generate the ser / de.
// impls.
impl TryFrom<String> for GedcomxDate {
    type Error = GedcomxError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl From<GedcomxDate> for String {
    fn from(gx: GedcomxDate) -> Self {
        gx.to_string()
    }
}

impl fmt::Display for GedcomxDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const INIT_CAPACITY: usize = 30;
        let mut s = String::with_capacity(INIT_CAPACITY);

        match self.0 {
            gedcomx_date::GedcomxDate::Simple(simple) => {
                if simple.approximate {
                    s.push('A');
                }

                date_time_into_string(&simple.date, &simple.time, &mut s);
            }
            gedcomx_date::GedcomxDate::Range(range) => {
                range_into_string(&range.start, &range.end, range.approximate, &mut s);
            }
            gedcomx_date::GedcomxDate::Recurring(recurring) => {
                if let Some(count) = recurring.count {
                    let _ = write!(s, "R{count}/");
                } else {
                    s.push_str("R/");
                }

                range_into_string(&Some(recurring.start), &Some(recurring.end), false, &mut s);
            }
        };

        write!(f, "{s}")
    }
}

fn date_time_into_string(
    date: &gedcomx_date::Date,
    time: &Option<gedcomx_date::Time>,
    s: &mut String,
) {
    s.push(if date.year >= 0 { '+' } else { '-' });
    let _ = write!(s, "{:04}", date.year.abs());

    if let Some(month) = date.month {
        let _ = write!(s, "-{month:02}");
    }

    if let Some(day) = date.day {
        let _ = write!(s, "-{day:02}");
    }

    if let Some(time) = time {
        let _ = write!(s, "T{:02}", time.hours);

        if let Some(minutes) = time.minutes {
            let _ = write!(s, ":{minutes:02}");
        }

        if let Some(seconds) = time.seconds {
            let _ = write!(s, ":{seconds:02}");
        }

        match (time.tz_offset_hours, time.tz_offset_minutes) {
            (Some(tz_hours), Some(tz_minutes)) => {
                if tz_hours == 0 && tz_minutes == 0 {
                    s.push('Z');
                } else {
                    s.push(if tz_hours > 0 || tz_minutes > 0 {
                        '+'
                    } else {
                        '-'
                    });
                    let _ = write!(s, "{:02}:{:02}", tz_hours.abs(), tz_minutes.abs());
                }
            }

            (Some(tz_hours), None) => {
                if tz_hours == 0 {
                    s.push('Z');
                } else {
                    s.push(if tz_hours > 0 { '+' } else { '-' });
                    let _ = write!(s, "{:02}", tz_hours.abs());
                }
            }

            _ => {
                // Includes a case where there are tz_minutes, but
                // no hours (not valid), and when there is no
                // timezone data (local time).
                // In both cases, just don't write anything out.
            }
        }
    }
}

fn range_into_string(
    start: &Option<gedcomx_date::DateTime>,
    end: &Option<gedcomx_date::DateTimeOrDuration>,
    approximate: bool,
    s: &mut String,
) {
    if approximate {
        s.push('A');
    }

    if let Some(start) = start {
        date_time_into_string(&start.date, &start.time, s);
    }

    s.push('/');

    match end {
        Some(gedcomx_date::DateTimeOrDuration::DateTime(datetime)) => {
            date_time_into_string(&datetime.date, &datetime.time, s);
        }
        Some(gedcomx_date::DateTimeOrDuration::Duration(duration)) => {
            duration_into_string(duration, s);
        }

        _ => {}
    }
}

fn duration_into_string(duration: &gedcomx_date::Duration, s: &mut String) {
    s.push('P');

    if duration.years != 0 {
        let _ = write!(s, "{}Y", duration.years);
    }

    if duration.months != 0 {
        let _ = write!(s, "{}M", duration.months);
    }

    if duration.days != 0 {
        let _ = write!(s, "{}D", duration.days);
    }

    if duration.hours != 0 || duration.minutes != 0 || duration.seconds != 0 {
        s.push('T');

        if duration.hours != 0 {
            let _ = write!(s, "{}H", duration.hours);
        }

        if duration.minutes != 0 {
            let _ = write!(s, "{}M", duration.minutes);
        }

        if duration.seconds != 0 {
            let _ = write!(s, "{}S", duration.seconds);
        }
    }
}

impl Arbitrary for GedcomxDate {
    fn arbitrary(g: &mut Gen) -> Self {
        let tz_offset_hours = arbitrary_between!(i32; g, -12, 12);
        let tz_offset_minutes = if tz_offset_hours < 0 {
            -arbitrary_between!(i32; g, 0, 59)
        } else {
            arbitrary_between!(i32; g, 0, 59)
        };

        let datetime_or_duration = vec![
            gedcomx_date::DateTimeOrDuration::DateTime(gedcomx_date::DateTime {
                date: gedcomx_date::Date {
                    year: arbitrary_between!(i32; g, 1800, 2200),
                    month: Some(arbitrary_between!(u32; g, 1, 12)),
                    day: Some(arbitrary_between!(u32; g, 1, 28)),
                },
                time: Some(gedcomx_date::Time {
                    hours: arbitrary_between!(u32; g, 0, 23),
                    minutes: Some(arbitrary_between!(u32; g, 0, 59)),
                    seconds: Some(arbitrary_between!(u32; g, 0, 59)),
                    tz_offset_hours: Some(tz_offset_hours),
                    tz_offset_minutes: Some(tz_offset_minutes),
                }),
            }),
            gedcomx_date::DateTimeOrDuration::Duration(gedcomx_date::Duration {
                years: arbitrary_between!(u32; g, 0, 400),
                months: arbitrary_between!(u32; g, 0, 11),
                days: arbitrary_between!(u32; g, 0, 28),
                hours: arbitrary_between!(u32; g, 0, 23),
                minutes: arbitrary_between!(u32; g, 0, 59),
                seconds: arbitrary_between!(u32; g, 0, 59),
            }),
        ];

        let options = vec![
            gedcomx_date::GedcomxDate::Simple(gedcomx_date::Simple {
                date: gedcomx_date::Date {
                    year: arbitrary_between!(i32; g, 1800, 2200),
                    month: Some(arbitrary_between!(u32; g, 1, 12)),
                    day: Some(arbitrary_between!(u32; g, 1, 28)),
                },
                time: Some(gedcomx_date::Time {
                    hours: arbitrary_between!(u32; g, 0, 23),
                    minutes: Some(arbitrary_between!(u32; g, 0, 59)),
                    seconds: Some(arbitrary_between!(u32; g, 0, 59)),
                    tz_offset_hours: Some(tz_offset_hours),
                    tz_offset_minutes: Some(tz_offset_minutes),
                }),
                approximate: bool::arbitrary(g),
            }),
            gedcomx_date::GedcomxDate::Range(gedcomx_date::Range {
                start: Some(gedcomx_date::DateTime {
                    date: gedcomx_date::Date {
                        year: arbitrary_between!(i32; g, 1800, 2200),
                        month: Some(arbitrary_between!(u32; g, 1, 12)),
                        day: Some(arbitrary_between!(u32; g, 1, 28)),
                    },
                    time: Some(gedcomx_date::Time {
                        hours: arbitrary_between!(u32; g, 0, 23),
                        minutes: Some(arbitrary_between!(u32; g, 0, 59)),
                        seconds: Some(arbitrary_between!(u32; g, 0, 59)),
                        tz_offset_hours: Some(tz_offset_hours),
                        tz_offset_minutes: Some(tz_offset_minutes),
                    }),
                }),
                end: Some(*g.choose(&datetime_or_duration).unwrap()),
                approximate: bool::arbitrary(g),
            }),
            gedcomx_date::GedcomxDate::Recurring(gedcomx_date::Recurring {
                start: gedcomx_date::DateTime {
                    date: gedcomx_date::Date {
                        year: arbitrary_between!(i32; g, 1800, 2200),
                        month: Some(arbitrary_between!(u32; g, 1, 12)),
                        day: Some(arbitrary_between!(u32; g, 1, 28)),
                    },
                    time: Some(gedcomx_date::Time {
                        hours: arbitrary_between!(u32; g, 0, 23),
                        minutes: Some(arbitrary_between!(u32; g, 0, 59)),
                        seconds: Some(arbitrary_between!(u32; g, 0, 59)),
                        tz_offset_hours: Some(tz_offset_hours),
                        tz_offset_minutes: Some(tz_offset_minutes),
                    }),
                },
                end: *g.choose(&datetime_or_duration).unwrap(),
                count: Some(u32::arbitrary(g)),
            }),
        ];

        Self(*g.choose(&options).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn roundtrip(s: String) {
        let original: GedcomxDate = s.parse().unwrap();
        let formal_string = original.to_string();
        let roundtrip: GedcomxDate = formal_string.parse().unwrap();
        assert_eq!(original, roundtrip);
    }

    #[test]
    fn roundtrip_simple_year() {
        roundtrip("+1000".to_string());
        roundtrip("-0010".to_string());
        roundtrip("-0000".to_string());
        roundtrip("+1000T10".to_string());
    }

    #[test]
    fn roundtrip_simple_month() {
        roundtrip("+0987-04".to_string());
        roundtrip("+1000-10T10".to_string());
    }

    #[test]
    fn roundtrip_simple_day() {
        roundtrip("+1600-02-29".to_string());
    }

    #[test]
    fn roundtrip_simple_minutes() {
        roundtrip("+1000-10-01T24:15".to_string());
        roundtrip("+0987-01-25T23:59".to_string());
        roundtrip("+0987-01-25T23:59Z".to_string());
    }

    #[test]
    fn roundtrip_simple_seconds() {
        roundtrip("+0987-01-25T23:59:59".to_string());
        roundtrip("+0987-01-25T23:59:59Z".to_string());
    }

    #[test]
    fn roundtrip_simple_timezones() {
        roundtrip("+1000-01-01T23:15Z".to_string());
        roundtrip("+1000-01-01T23:15+15".to_string());
        roundtrip("+1000-01-01T23:15-02".to_string());
        roundtrip("+1000-01-01T23:15-00".to_string());
        roundtrip("+1000-01-01T23:15-00:30".to_string());
    }

    #[test]
    fn roundtrip_simple_approximate() {
        roundtrip("A+0987-01-25T24Z".to_string());
        roundtrip("A+0987".to_string());
    }

    #[test]
    fn roundtrip_recurring() {
        roundtrip("R/+1000/+2000-10-01".to_string());
        roundtrip("R3/+1000/+2000-10-01".to_string());
        roundtrip("R/+1000/P1Y2M3DT4H5M6S".to_string());
    }

    #[test]
    fn roundtrip_range() {
        roundtrip("+1000/P1Y2M3DT4H5M6S".to_string());
        roundtrip("+1000/+2000-10-01".to_string());
        roundtrip("/+2000-10-01".to_string());
        roundtrip("+1000/".to_string());
        roundtrip("A+1000/+2000-10-01".to_string());
    }
}

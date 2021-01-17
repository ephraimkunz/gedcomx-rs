use crate::components::{ResourceReference, Timestamp};
use chrono::serde::ts_milliseconds_option;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Attribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor: Option<ResourceReference>,

    #[serde(
        default,
        with = "ts_milliseconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub modified: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<ResourceReference>,

    #[serde(
        default,
        with = "ts_milliseconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<Timestamp>,
}

impl Attribution {
    pub fn new() -> Self {
        Self {
            contributor: None,
            change_message: None,
            modified: None,
            creator: None,
            created: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "contributor" : {
                "resource" : "http://identifier/for/contributor"
              },
              "modified" : 1338494969,
              "changeMessage" : "...change message here...",
              "creator" : {
                "resource" : "http://identifier/for/creator"
              },
              "created" : 1338394969
        }"#;

        let attribution: Attribution = serde_json::from_str(json).unwrap();
        assert_eq!(
            attribution,
            Attribution {
                contributor: Some(ResourceReference::from("http://identifier/for/contributor")),
                modified: Some(chrono::DateTime::from_utc(
                    chrono::NaiveDateTime::from_timestamp(1338494, 969000000),
                    chrono::Utc
                )),
                change_message: Some(String::from("...change message here...")),
                creator: Some(ResourceReference::from("http://identifier/for/creator")),
                created: Some(chrono::DateTime::from_utc(
                    chrono::NaiveDateTime::from_timestamp(1338394, 969000000),
                    chrono::Utc
                )),
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{}"#;

        let attribution: Attribution = serde_json::from_str(json).unwrap();
        assert_eq!(attribution, Attribution::new())
    }

    #[test]
    fn json_serialize() {
        let attribution = Attribution {
            contributor: Some(ResourceReference::from("http://identifier/for/contributor")),
            modified: Some(chrono::DateTime::from_utc(
                chrono::NaiveDateTime::from_timestamp(1338494, 969000000),
                chrono::Utc,
            )),
            change_message: Some(String::from("...change message here...")),
            creator: Some(ResourceReference::from("http://identifier/for/creator")),
            created: Some(chrono::DateTime::from_utc(
                chrono::NaiveDateTime::from_timestamp(1338394, 969000000),
                chrono::Utc,
            )),
        };

        let json = serde_json::to_string(&attribution).unwrap();

        assert_eq!(
            json,
            r#"{"contributor":{"resource":"http://identifier/for/contributor"},"modified":1338494969,"changeMessage":"...change message here...","creator":{"resource":"http://identifier/for/creator"},"created":1338394969}"#
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let attribution = Attribution::new();

        let json = serde_json::to_string(&attribution).unwrap();

        assert_eq!(json, r#"{}"#);
    }
}

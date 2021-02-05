use crate::{Agent, ResourceReference, Result, Timestamp};
use chrono::serde::ts_milliseconds_option;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::convert::TryInto;

/// The data structure used to attribute who, when, and why to genealogical data.
///
/// Data is attributed to the agent who made the latest significant change to the nature of the data being attributed.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Attribution {
    /// Reference to the agent to whom the attributed data is attributed. If provided, MUST resolve to an instance of [`Agent`](crate::Agent).
    pub contributor: Option<ResourceReference>, // TODO: Enforce this constraint?

    /// Timestamp of when the attributed data was modified.
    #[serde(default, with = "ts_milliseconds_option")]
    pub modified: Option<Timestamp>,

    /// A statement of why the attributed data is being provided by the contributor.
    pub change_message: Option<String>,

    /// Reference to the agent that created the attributed data. The creator MAY be different from the contributor
    /// if changes were made to the attributed data. If provided, MUST resolve to an instance of [`Agent`](crate::Agent).
    pub creator: Option<ResourceReference>,

    /// Timestamp of when the attributed data was contributed.
    #[serde(default, with = "ts_milliseconds_option")]
    pub created: Option<Timestamp>,
}

impl Attribution {
    pub fn new(
        contributor: Option<ResourceReference>,
        modified: Option<Timestamp>,
        change_message: Option<String>,
        creator: Option<ResourceReference>,
        created: Option<Timestamp>,
    ) -> Self {
        Self {
            contributor,
            modified,
            change_message,
            creator,
            created,
        }
    }

    pub fn builder() -> AttributionBuilder {
        AttributionBuilder::new()
    }
}

pub struct AttributionBuilder(Attribution);

impl AttributionBuilder {
    pub(crate) fn new() -> Self {
        Self(Attribution::default())
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `agent` has no `id` set.
    pub fn contributor(&mut self, agent: &Agent) -> Result<&mut Self> {
        self.0.contributor = Some(agent.try_into()?);
        Ok(self)
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `agent` has no `id` set.
    pub fn creator(&mut self, agent: &Agent) -> Result<&mut Self> {
        self.0.creator = Some(agent.try_into()?);
        Ok(self)
    }

    pub fn modified<I: Into<Timestamp>>(&mut self, timestamp: I) -> &mut Self {
        self.0.modified = Some(timestamp.into());
        self
    }

    pub fn created<I: Into<Timestamp>>(&mut self, timestamp: I) -> &mut Self {
        self.0.created = Some(timestamp.into());
        self
    }

    pub fn change_message<I: Into<String>>(&mut self, change_message: I) -> &mut Self {
        self.0.change_message = Some(change_message.into());
        self
    }

    pub fn build(&self) -> Attribution {
        Attribution::new(
            self.0.contributor.clone(),
            self.0.modified,
            self.0.change_message.clone(),
            self.0.creator.clone(),
            self.0.created,
        )
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
        assert_eq!(attribution, Attribution::default())
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
        let attribution = Attribution::default();

        let json = serde_json::to_string(&attribution).unwrap();

        assert_eq!(json, r#"{}"#);
    }
}

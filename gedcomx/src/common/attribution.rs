use std::convert::TryInto;

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Agent, ResourceReference, Result, Timestamp};

/// The data structure used to attribute who, when, and why to genealogical
/// data.
///
/// Data is attributed to the agent who made the latest significant change to
/// the nature of the data being attributed.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    rename = "attribution",
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Attribution {
    /// Reference to the agent to whom the attributed data is attributed. If
    /// provided, MUST resolve to an instance of [`Agent`](crate::Agent).
    #[yaserde(prefix = "gx")]
    pub contributor: Option<ResourceReference>,

    /// Timestamp of when the attributed data was modified.
    #[yaserde(prefix = "gx")]
    pub modified: Option<Timestamp>,

    /// A statement of why the attributed data is being provided by the
    /// contributor.
    #[yaserde(rename = "changeMessage", prefix = "gx")]
    pub change_message: Option<String>,

    /// Reference to the agent that created the attributed data. The creator MAY
    /// be different from the contributor if changes were made to the
    /// attributed data. If provided, MUST resolve to an instance of
    /// [`Agent`](crate::Agent).
    #[yaserde(prefix = "gx")]
    pub creator: Option<ResourceReference>,

    /// Timestamp of when the attributed data was contributed.
    #[yaserde(prefix = "gx")]
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

impl Arbitrary for Attribution {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut attribution = Self::builder()
            .modified(Timestamp::arbitrary(g))
            .change_message(crate::arbitrary_trimmed(g))
            .created(Timestamp::arbitrary(g))
            .build();

        attribution.contributor = Some(ResourceReference::arbitrary(g));
        attribution.creator = Some(ResourceReference::arbitrary(g));

        attribution
    }
}

pub struct AttributionBuilder(Attribution);

impl AttributionBuilder {
    pub(crate) fn new() -> Self {
        Self(Attribution::default())
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `agent` has no `id` set.
    pub fn contributor(&mut self, agent: &Agent) -> Result<&mut Self> {
        self.0.contributor = Some(agent.try_into()?);
        Ok(self)
    }

    pub fn modified<I: Into<Timestamp>>(&mut self, timestamp: I) -> &mut Self {
        self.0.modified = Some(timestamp.into());
        self
    }

    pub fn change_message<I: Into<String>>(&mut self, change_message: I) -> &mut Self {
        self.0.change_message = Some(change_message.into());
        self
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `agent` has no `id` set.
    pub fn creator(&mut self, agent: &Agent) -> Result<&mut Self> {
        self.0.creator = Some(agent.try_into()?);
        Ok(self)
    }

    pub fn created<I: Into<Timestamp>>(&mut self, timestamp: I) -> &mut Self {
        self.0.created = Some(timestamp.into());
        self
    }

    pub fn build(&self) -> Attribution {
        Attribution::new(
            self.0.contributor.clone(),
            self.0.modified.clone(),
            self.0.change_message.clone(),
            self.0.creator.clone(),
            self.0.created.clone(),
        )
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use yaserde::ser::Config;

    use super::*;
    use crate::GedcomxError;

    #[test]
    fn builder() {
        let creator = Agent::builder().id("creator").build();
        let contributor = Agent::builder().id("contributor").build();

        let expected = Attribution {
            contributor: Some((&contributor).try_into().unwrap()),
            modified: Some(Timestamp::default()),
            change_message: Some("change message".to_string()),
            creator: Some((&creator).try_into().unwrap()),
            created: Some(Timestamp::default()),
        };

        let actual = Attribution::builder()
            .contributor(&contributor)
            .unwrap()
            .modified(Timestamp::default())
            .change_message("change message")
            .creator(&creator)
            .unwrap()
            .created(Timestamp::default())
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn builder_fails_correctly() {
        let creator = Agent::default();
        let contributor = Agent::default();

        let actual = Attribution::builder()
            .contributor(&contributor)
            .map(|b| b.build());
        assert_eq!(
            actual.unwrap_err().to_string(),
            GedcomxError::no_id_error(&contributor).to_string()
        );

        let actual = Attribution::builder().creator(&creator).map(|b| b.build());
        assert_eq!(
            actual.unwrap_err().to_string(),
            GedcomxError::no_id_error(&creator).to_string()
        );
    }

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
                modified: Some(
                    chrono::DateTime::from_utc(
                        chrono::NaiveDateTime::from_timestamp_opt(1_338_494, 969_000_000)
                            .expect("Invalid date"),
                        chrono::Utc
                    )
                    .into()
                ),
                change_message: Some(String::from("...change message here...")),
                creator: Some(ResourceReference::from("http://identifier/for/creator")),
                created: Some(
                    chrono::DateTime::from_utc(
                        chrono::NaiveDateTime::from_timestamp_opt(1_338_394, 969_000_000)
                            .expect("Invalid date"),
                        chrono::Utc
                    )
                    .into()
                ),
            }
        );
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{}"#;

        let attribution: Attribution = serde_json::from_str(json).unwrap();
        assert_eq!(attribution, Attribution::default());
    }

    #[test]
    fn json_serialize() {
        let attribution = Attribution {
            contributor: Some(ResourceReference::from("http://identifier/for/contributor")),
            modified: Some(
                chrono::DateTime::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(1_338_494, 969_000_000)
                        .expect("Invalid date"),
                    chrono::Utc,
                )
                .into(),
            ),
            change_message: Some(String::from("...change message here...")),
            creator: Some(ResourceReference::from("http://identifier/for/creator")),
            created: Some(
                chrono::DateTime::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(1_338_394, 969_000_000)
                        .expect("Invalid date"),
                    chrono::Utc,
                )
                .into(),
            ),
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

    #[test]
    fn xml_deserialize() {
        let xml = r#"
        <attribution>
            <contributor resource="http://identifier/for/contributor"/>
            <modified>2012-06-29T00:00:00</modified>
            <changeMessage>...change message here...</changeMessage>
            <creator resource="http://identifier/for/creator"/>
            <created>2012-05-29T00:00:00</created>
          </attribution>"#;

        let attribution: Attribution = yaserde::de::from_str(xml).unwrap();

        let expected_attribution = Attribution {
            contributor: Some("http://identifier/for/contributor".into()),
            modified: Some("2012-06-29T00:00:00".parse().unwrap()),
            change_message: Some("...change message here...".to_string()),
            creator: Some("http://identifier/for/creator".into()),
            created: Some("2012-05-29T00:00:00".parse().unwrap()),
        };

        assert_eq!(attribution, expected_attribution);
    }

    #[test]
    fn xml_deserialize_optional_fields() {
        let xml = r#"
        <attribution>
        </attribution>"#;

        let attribution: Attribution = yaserde::de::from_str(xml).unwrap();

        let expected_attribution = Attribution::default();
        assert_eq!(attribution, expected_attribution);
    }

    #[test]
    fn xml_serialize() {
        let attribution = Attribution {
            contributor: Some("http://identifier/for/contributor".into()),
            modified: Some("2012-06-29T00:00:00".parse().unwrap()),
            change_message: Some("...change message here...".to_string()),
            creator: Some("http://identifier/for/creator".into()),
            created: Some("2012-05-29T00:00:00".parse().unwrap()),
        };

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };
        let xml = yaserde::ser::to_string_with_config(&attribution, &config).unwrap();

        let expected_xml = r#"<attribution xmlns="http://gedcomx.org/v1/"><contributor resource="http://identifier/for/contributor" /><modified>2012-06-29T00:00:00</modified><changeMessage>...change message here...</changeMessage><creator resource="http://identifier/for/creator" /><created>2012-05-29T00:00:00</created></attribution>"#;

        assert_eq!(xml, expected_xml);
    }

    #[test]
    fn xml_serialize_optional_fields() {
        let attribution = Attribution::default();

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };
        let xml = yaserde::ser::to_string_with_config(&attribution, &config).unwrap();

        let expected_xml = r#"<attribution xmlns="http://gedcomx.org/v1/" />"#;

        assert_eq!(xml, expected_xml);
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: Attribution) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: Attribution = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: Attribution) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: Attribution = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

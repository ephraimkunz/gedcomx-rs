#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
// #![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]

pub extern crate gedcomx_date;

#[macro_use]
mod macros;

mod agent;
pub use agent::*;

mod common;
pub use common::*;

mod conclusion;
pub use conclusion::*;

mod error;
pub use error::GedcomxError;

mod gedcomx;
pub use crate::gedcomx::*;

mod source;
use std::fmt;

use serde::{Deserialize, Serialize};
pub use source::*;

pub type Result<T> = std::result::Result<T, GedcomxError>;

// I can't figure out how to get Serde to properly serialize enums with a bunch
// of normal variants and then one catch-all variant that includes the string
// from the json, just using attributes. So, rather than write a Deserializer /
// Serializer implementation we'll just serialize to this newtype and then Serde
// will automatically  convert it to the required type.
#[derive(Serialize, Deserialize, Clone)]
struct EnumAsString(String);

impl<T: fmt::Display> From<T> for EnumAsString {
    fn from(t: T) -> Self {
        Self(t.to_string())
    }
}

#[cfg(test)]
struct TestConclusionData {
    pub id: Option<Id>,
    pub lang: Option<Lang>,
    pub sources: Vec<SourceReference>,
    pub analysis: Option<ResourceReference>,
    pub notes: Vec<Note>,
    pub confidence: Option<ConfidenceLevel>,
    pub attribution: Option<Attribution>,
}

#[cfg(test)]
struct TestData {
    attribution: Attribution,
    source_reference: SourceReference,
    note: Note,
    evidence_reference: EvidenceReference,
    conclusion_data: TestConclusionData,
}

#[cfg(test)]
impl TestData {
    #[allow(dead_code)]
    fn new() -> Self {
        let attribution = Attribution {
            contributor: Some(ResourceReference::from("A-1")),
            modified: Some(
                chrono::DateTime::from_utc(
                    chrono::NaiveDateTime::from_timestamp(1_394_175_600, 0),
                    chrono::Utc,
                )
                .into(),
            ),
            ..Attribution::default()
        };

        let qualifier = Qualifier {
            name: SourceReferenceQualifier::RectangleRegion.into(),
            value: Some("rectangle region value".into()),
        };
        let mut source_reference = SourceReference::new(Uri::from("SD-1"), None, None, vec![]);
        source_reference.description_id = Some("Description id of the target source".into());
        source_reference.attribution = Some(attribution.clone());
        source_reference.qualifiers = vec![qualifier];

        let note = Note::builder("This is a note")
            .lang("en")
            .subject("subject")
            .attribution(attribution.clone())
            .build();

        let conclusion_data = TestConclusionData {
            id: Some("local_id".into()),
            lang: Some("en".into()),
            sources: vec![source_reference.clone()],
            analysis: Some(ResourceReference::from(
                "http://identifier/for/analysis/document",
            )),
            notes: vec![note.clone()],
            confidence: Some(ConfidenceLevel::High),
            attribution: Some(attribution.clone()),
        };

        let mut evidence_reference = EvidenceReference::new(Uri::from("S-1"), None);
        evidence_reference.attribution = Some(attribution.clone());

        Self {
            attribution,
            source_reference,
            note,
            evidence_reference,
            conclusion_data,
        }
    }
}

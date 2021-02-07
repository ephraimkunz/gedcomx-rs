#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
// #![deny(clippy::cargo)]
// #![deny(missing_docs)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::new_without_default)]
#![allow(clippy::pub_enum_variant_names)]
#![allow(clippy::clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]

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
pub use source::*;

use serde::{Deserialize, Serialize};
use std::fmt;

pub type Result<T> = std::result::Result<T, GedcomxError>;

// I can't figure out how to get Serde to properly serialize enums with a bunch of normal variants and then
// one catch-all variant that includes the string from the json, just using attributes. So, rather than write a
// Deserializer / Serializer implementation we'll just serialize to this newtype and then Serde will automatically
//  convert it to the required type.
#[derive(Serialize, Deserialize)]
struct EnumAsString(String);

impl<T: fmt::Display> From<T> for EnumAsString {
    fn from(t: T) -> Self {
        Self(t.to_string())
    }
}

#[allow(dead_code)]
struct TestData {
    attribution: Attribution,
    source_reference: SourceReference,
    note: Note,
    conclusion_data: ConclusionData,
    evidence_reference: EvidenceReference,
    subject_data: SubjectData,
}

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
            value: Some("rectangle region value".to_string()),
        };
        let mut source_reference = SourceReference::builder_with_raw(Uri::from("SD-1")).build();
        source_reference.description_id = Some("Description id of the target source".into());
        source_reference.attribution = Some(attribution.clone());
        source_reference.qualifiers = vec![qualifier];

        let mut note = Note::new("This is a note".to_string());
        note.attribution = Some(attribution.clone());
        note.lang = Some("en".into());
        note.subject = Some("subject".to_string());

        let mut conclusion_data = ConclusionData::new();
        conclusion_data.id = Some("local_id".into());
        conclusion_data.lang = Some("en".into());
        conclusion_data.sources = vec![source_reference.clone()];
        conclusion_data.analysis = Some(ResourceReference::from(
            "http://identifier/for/analysis/document",
        ));
        conclusion_data.notes = vec![note.clone()];
        conclusion_data.confidence = Some(ConfidenceLevel::High);
        conclusion_data.attribution = Some(attribution.clone());

        let mut evidence_reference = EvidenceReference::new(Uri::from("S-1"), None);
        evidence_reference.attribution = Some(attribution.clone());

        let mut subject_data = SubjectData::new(conclusion_data.clone());
        subject_data.extracted = Some(false);
        subject_data.evidence = vec![evidence_reference.clone()];
        subject_data.media = vec![source_reference.clone()];
        subject_data.identifiers = vec![]; // TODO: Empty until I get this serializing properly.
        subject_data.conclusion = conclusion_data.clone();

        Self {
            attribution,
            source_reference,
            note,
            conclusion_data,
            evidence_reference,
            subject_data,
        }
    }
}

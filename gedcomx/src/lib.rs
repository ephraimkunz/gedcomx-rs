//! # GEDCOM X
//! GEDCOM X defines an open data model and an open serialization format for
//! exchanging the genealogical data essential to the genealogical research
//! process. Examples of these data include information about persons, the
//! relationships between persons, and the records that support that
//! information.
//!
//! This library provides the base types necessary to implement the GEDCOM X
//! conceptual model and aims to implement further extensions to this model in
//! the future. The goal of the library is to provide a correct, hard to misuse
//! API that can handle serialization / deserialization to and from both
//! JSON and XML. It's intended to be a solid foundationalal building block that
//! other genealogical software can be built on top of.
//!
//! ## Deserialize a GEDCOM X document from JSON
//!
//! ```
//! use gedcomx_date::{parse, GedcomxDate};
//! let date = parse("+1988-03-29T03:19").unwrap();
//! match date {
//!     GedcomxDate::Simple(simple_date) => {
//!         let date = simple_date.date;
//!         println!("{}", date.year); // 1988
//!         println!("{}", date.month.unwrap()); // 3
//!         println!("{}", date.day.unwrap()); // 29
//!         let time = simple_date.time.unwrap();
//!         println!("{}", time.hours); // 3
//!         println!("{}", time.minutes.unwrap()); // 19
//!     }
//!     _ => {}
//! }
//! ```

//! ## Build and serialize a GEDCOM X document to JSON
//!
//! ```
//! use gedcomx_date::{parse, GedcomxDate};
//! let date = parse("+1988-03-29T03:19").unwrap();
//! match date {
//!     GedcomxDate::Simple(simple_date) => {
//!         let date = simple_date.date;
//!         println!("{}", date.year); // 1988
//!         println!("{}", date.month.unwrap()); // 3
//!         println!("{}", date.day.unwrap()); // 29
//!         let time = simple_date.time.unwrap();
//!         println!("{}", time.hours); // 3
//!         println!("{}", time.minutes.unwrap()); // 19
//!     }
//!     _ => {}
//! }
//! ```

//! ## Deserialize a GEDCOM X document from XML
//!
//! ```
//! use gedcomx_date::{parse, GedcomxDate};
//! let date = parse("+1988-03-29T03:19").unwrap();
//! match date {
//!     GedcomxDate::Simple(simple_date) => {
//!         let date = simple_date.date;
//!         println!("{}", date.year); // 1988
//!         println!("{}", date.month.unwrap()); // 3
//!         println!("{}", date.day.unwrap()); // 29
//!         let time = simple_date.time.unwrap();
//!         println!("{}", time.hours); // 3
//!         println!("{}", time.minutes.unwrap()); // 19
//!     }
//!     _ => {}
//! }
//! ```

//! ## Build and serialize a GEDCOM X document to XML
//!
//! ```
//! use gedcomx_date::{parse, GedcomxDate};
//! let date = parse("+1988-03-29T03:19").unwrap();
//! match date {
//!     GedcomxDate::Simple(simple_date) => {
//!         let date = simple_date.date;
//!         println!("{}", date.year); // 1988
//!         println!("{}", date.month.unwrap()); // 3
//!         println!("{}", date.day.unwrap()); // 29
//!         let time = simple_date.time.unwrap();
//!         println!("{}", time.hours); // 3
//!         println!("{}", time.minutes.unwrap()); // 19
//!     }
//!     _ => {}
//! }
//! ```

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
// #![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![allow(clippy::nonstandard_macro_braces)] // https://github.com/rust-lang/rust-clippy/issues/7434
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
            notes: vec![note],
            confidence: Some(ConfidenceLevel::High),
            attribution: Some(attribution.clone()),
        };

        let mut evidence_reference = EvidenceReference::new(Uri::from("S-1"), None);
        evidence_reference.attribution = Some(attribution.clone());

        Self {
            attribution,
            source_reference,
            evidence_reference,
            conclusion_data,
        }
    }
}

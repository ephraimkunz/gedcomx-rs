//! # GEDCOM X
//! GEDCOM X defines an open data model and an open serialization format for
//! exchanging the genealogical data essential to the genealogical research
//! process. Examples of these data include information about persons, the
//! relationships between persons, and the records that support that
//! information.
//!
//! This library provides the base types necessary to implement the GEDCOM X
//! conceptual model and aims to implement further extensions to this model in
//! the future. The goal of the library is to provide a correct, hard-to-misuse
//! API that can handle serialization / deserialization to and from both
//! JSON and XML. It's intended to be a solid foundational building block that
//! other genealogical software can be built on top of.
//!
//! # Examples
//! ## Deserialize a GEDCOM X document from JSON
//!
//! ```
//! use gedcomx::Gedcomx;
//!
//! let json = std::fs::read_to_string("../data/birth.json").unwrap();
//! let gx = Gedcomx::from_json_str(&json).unwrap();
//! println!(
//!     "Successfully deserialized GEDCOM X document from JSON with {} people inside!",
//!     gx.persons.len()
//! );
//!
//! assert_eq!(gx.persons.len(), 4);
//! ```

//! ## Build and serialize a GEDCOM X document to JSON
//! Most of the GEDCOM X types have lots of properties. Builders are provided
//! for most types to conveniently set only the properties you choose to.
//! Builders can be created with the `builder` method on the specific type. This
//! method will take any required argument the final type needs to have set.
//! Other properties can then be set on the builder. After the builder has been
//! fully configured, it can be transformed into an instance of the type it is
//! building by calling the `build` method on it.
//!
//! ```
//! use gedcomx::{Gedcomx, Name, NameForm, NameType, Person};
//!
//! let gx = Gedcomx::builder()
//!     .person(
//!         Person::builder()
//!             .private(true)
//!             .name(
//!                 Name::builder(
//!                     NameForm::builder()
//!                         .full_text("Jim Halpert")
//!                         .lang("en")
//!                         .build(),
//!                 )
//!                 .name_type(NameType::BirthName)
//!                 .build(),
//!             )
//!             .build(),
//!     )
//!     .build();
//!
//! let json = gx.to_json_string_pretty().unwrap();
//!
//! assert_eq!(json.len(), 285);
//! ```

//! ## Deserialize a GEDCOM X document from XML
//!
//! ```
//! use gedcomx::Gedcomx;
//!
//! let xml = std::fs::read_to_string("../data/birth.xml").unwrap();
//! let gx = Gedcomx::from_xml_str(&xml).unwrap();
//! println!(
//!     "Successfully deserialized GEDCOM X document from XML with {} people inside!",
//!     gx.persons.len()
//! );
//!
//! assert_eq!(gx.persons.len(), 4);
//! ```

//! ## Build and serialize a GEDCOM X document to XML
//!
//! ```
//! use gedcomx::{Gedcomx, Name, NameForm, NameType, Person};
//!
//! let gx = Gedcomx::builder()
//!     .person(
//!         Person::builder()
//!             .private(true)
//!             .name(
//!                 Name::builder(
//!                     NameForm::builder()
//!                         .full_text("Jim Halpert")
//!                         .lang("en")
//!                         .build(),
//!                 )
//!                 .name_type(NameType::BirthName)
//!                 .build(),
//!             )
//!             .build(),
//!     )
//!     .build();
//!
//! let xml = gx.to_xml_string_pretty().unwrap();
//!
//! assert_eq!(xml.len(), 277);
//! ```

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
// #![deny(clippy::cargo)]
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
                chrono::DateTime::from_timestamp(1_394_175_600, 0)
                    .expect("Invalid date")
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

// XML roundtripping for Option<String> is only guaranteeed when a string is
// non-empty and has leading and trailing whitespace trimmed. If the string is
// empty, it will be roundtripped to None. If it has leading or trailing
// whitespace, that will be stripped due to yaserde setting the trim_whitespace
// flag on the wrapped xml parser.
// Also strip out control characters, since most aren't valid XML and we want
// our tests to mainly check roundtripping of XML, not failure to parse
// disallowed chars.
use quickcheck::{Arbitrary, Gen};

fn arbitrary_trimmed(g: &mut Gen) -> String {
    let mut trimmed = String::new();
    while trimmed.is_empty() {
        let non_trimmed = String::arbitrary(g);
        trimmed = non_trimmed
            .chars()
            .filter(|&c| xml::common::is_xml10_char(c) && c != ']')
            .collect::<String>()
            .trim()
            .to_string();
    }

    trimmed
}

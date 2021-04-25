#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
// #![deny(clippy::cargo)]
// #![deny(missing_docs)]
// #![deny(missing_doc_code_examples)]
#![allow(clippy::clippy::too_many_arguments)]
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

mod serde_vec_identifier_to_map {
    use std::{collections::HashMap, fmt};

    use serde::{
        de::{Deserializer, MapAccess, Visitor},
        ser::{SerializeMap, Serializer},
    };

    use crate::{EnumAsString, Identifier, Uri};

    pub fn serialize<S>(identifiers: &[Identifier], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hashmap: HashMap<String, Vec<Uri>> = HashMap::with_capacity(identifiers.len());
        for id in identifiers {
            let e = hashmap
                .entry(
                    id.identifier_type
                        .as_ref()
                        .map_or(String::from("$"), std::string::ToString::to_string),
                )
                .or_insert_with(Vec::new);
            e.push(id.value.clone());
            e.sort_by_key(std::string::ToString::to_string)
        }

        let mut map = serializer.serialize_map(Some(identifiers.len()))?;

        let mut keys = hashmap.keys().collect::<Vec<_>>();
        keys.sort();

        for k in keys {
            map.serialize_entry(k, hashmap.get(k).unwrap())?;
        }

        map.end()
    }

    struct IdentifierVisitor;

    impl<'de> Visitor<'de> for IdentifierVisitor {
        // The type that our Visitor is going to produce.
        type Value = Vec<Identifier>;

        // Format a message stating what data this Visitor expects to receive.
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("A map of identifier types to lists of values")
        }

        // Deserialize Value from an abstract "map" provided by the
        // Deserializer. The MapAccess input is a callback provided by
        // the Deserializer to let us see each entry in the map.
        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut vec = Vec::with_capacity(access.size_hint().unwrap_or(0));

            // While there are entries remaining in the input, add them
            // into our vec.
            while let Some((key, value)) = access.next_entry::<EnumAsString, Vec<Uri>>()? {
                for v in value {
                    let k = if key.0 == "$" {
                        None
                    } else {
                        Some(key.clone().into())
                    };
                    vec.push(Identifier::new(v, k));
                }
            }

            Ok(vec)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Identifier>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_map(IdentifierVisitor {})
    }
}

struct TestConclusionData {
    pub id: Option<Id>,
    pub lang: Option<Lang>,
    pub sources: Vec<SourceReference>,
    pub analysis: Option<ResourceReference>,
    pub notes: Vec<Note>,
    pub confidence: Option<ConfidenceLevel>,
    pub attribution: Option<Attribution>,
}

#[allow(dead_code)]
struct TestData {
    attribution: Attribution,
    source_reference: SourceReference,
    note: Note,
    evidence_reference: EvidenceReference,
    conclusion_data: TestConclusionData,
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
            value: Some("rectangle region value".into()),
        };
        let mut source_reference = SourceReference::builder_with_raw(Uri::from("SD-1")).build();
        source_reference.description_id = Some("Description id of the target source".into());
        source_reference.attribution = Some(attribution.clone());
        source_reference.qualifiers = vec![qualifier];

        let mut note = Note::new("This is a note".to_string());
        note.attribution = Some(attribution.clone());
        note.lang = Some("en".into());
        note.subject = Some("subject".to_string());

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
            conclusion_data,
            evidence_reference,
        }
    }
}

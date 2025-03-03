use std::convert::TryFrom;

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Attribution, Event, GedcomxError, Group, Person, PlaceDescription, Relationship, Uri};

/// A reference to data being used to derive the given instance of Subject.
///
/// For example, an "evidence" Subject (i.e., the object holding the
/// `EvidenceReference` instance) can refer to content extracted from a source
/// (i.e., an "extracted" Subject) as information being used to derive the
/// evidence expressed in this Subject.
///
/// # Examples
/// An application allows a researcher to extract information from a single
/// census record about a person, representing the information as a persona with
/// an identifier "abcde". The researcher extracts additional information about
/// the person from a birth certificate and the application assigns
/// the resulting persona an identifier "fghij". As the researcher gathers and
/// analyzes the information, he will create a (working) `Person` conclusion.
/// When the researcher concludes that the person represented in "abcde" and in
/// "fghij" are the same person, he will add two `EvidenceReference` instances
/// to the working `Person`: one for "abcde" and one for "fghij".
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    rename = "evidence",
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct EvidenceReference {
    /// Reference to the supporting data.
    #[yaserde(attribute)]
    pub resource: Uri,

    /// The attribution of this evidence reference. If not provided, the
    /// attribution of the containing resource of the source reference is
    /// assumed.
    #[yaserde(prefix = "gx")]
    pub attribution: Option<Attribution>,
}

impl EvidenceReference {
    pub fn new(resource: Uri, attribution: Option<Attribution>) -> Self {
        Self {
            resource,
            attribution,
        }
    }
}

// Ideally we'd implement all the TryFroms with a blanket imple like impl <T:
// Subject> TryFrom<&T> for EvidenceReference. But that doesn't work due to https://github.com/rust-lang/rust/issues/50133. So insead we'll implement them with this macro.

try_from_evidencereference!(Person);
try_from_evidencereference!(Event);
try_from_evidencereference!(PlaceDescription);
try_from_evidencereference!(Relationship);
try_from_evidencereference!(Group);

impl Arbitrary for EvidenceReference {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(Uri::arbitrary(g), Some(Attribution::arbitrary(g)))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use yaserde::ser::Config;

    use super::*;
    use crate::TestData;

    #[test]
    fn json_deserialize() {
        let data = TestData::new();

        let json = r#"{
            "resource" : "S-1",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            }        
        }"#;

        let evidence_reference: EvidenceReference = serde_json::from_str(json).unwrap();
        assert_eq!(evidence_reference, data.evidence_reference);
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "resource" : "S-1"   
        }"#;

        let evidence_reference: EvidenceReference = serde_json::from_str(json).unwrap();
        assert_eq!(
            evidence_reference,
            EvidenceReference::new(Uri::from("S-1"), None)
        );
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let evidence_reference = data.evidence_reference;

        let json = serde_json::to_string(&evidence_reference).unwrap();

        assert_eq!(
            json,
            r#"{"resource":"S-1","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}"#
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let evidence_reference = EvidenceReference::new(Uri::from("S-1"), None);

        let json = serde_json::to_string(&evidence_reference).unwrap();

        assert_eq!(json, r#"{"resource":"S-1"}"#);
    }

    #[test]
    fn xml_deserialize() {
        let xml = r#"
        <evidence xmlns="http://gedcomx.org/v1/" resource="http://identifier/for/data/being/referenced">
            <attribution>
            </attribution>
        </evidence>"#;

        let evidence_reference: EvidenceReference = yaserde::de::from_str(xml).unwrap();

        let expected_evidence_reference = EvidenceReference {
            resource: "http://identifier/for/data/being/referenced".into(),
            attribution: Some(Attribution::default()),
        };

        assert_eq!(evidence_reference, expected_evidence_reference);
    }

    #[test]
    fn xml_deserialize_optional_fields() {
        let xml = r#"
        <evidence resource="http://identifier/for/data/being/referenced">
        </evidence>"#;

        let evidence_reference: EvidenceReference = yaserde::de::from_str(xml).unwrap();

        let expected_evidence_reference = EvidenceReference {
            resource: "http://identifier/for/data/being/referenced".into(),
            attribution: None,
        };

        assert_eq!(evidence_reference, expected_evidence_reference);
    }

    #[test]
    fn xml_serialize() {
        let evidence_reference = EvidenceReference {
            resource: "http://identifier/for/data/being/referenced".into(),
            attribution: Some(Attribution::default()),
        };

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };
        let xml = yaserde::ser::to_string_with_config(&evidence_reference, &config).unwrap();

        let expected_xml = r#"<evidence xmlns="http://gedcomx.org/v1/" resource="http://identifier/for/data/being/referenced"><attribution /></evidence>"#;

        assert_eq!(xml, expected_xml);
    }

    #[test]
    fn xml_serialize_optional_fields() {
        let evidence_reference = EvidenceReference {
            resource: "http://identifier/for/data/being/referenced".into(),
            attribution: None,
        };

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };

        let xml = yaserde::ser::to_string_with_config(&evidence_reference, &config).unwrap();

        let expected_xml = r#"<evidence xmlns="http://gedcomx.org/v1/" resource="http://identifier/for/data/being/referenced" />"#;

        assert_eq!(xml, expected_xml);
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: EvidenceReference) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: EvidenceReference = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: EvidenceReference) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: EvidenceReference = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

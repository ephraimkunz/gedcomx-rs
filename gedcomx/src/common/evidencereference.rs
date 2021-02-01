use crate::{Attribution, Event, GedcomxError, Person, PlaceDescription, Relationship, Uri};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// A reference to data being used to derive the given instance of Subject.
///
/// For example, an "evidence" Subject (i.e., the object holding the `EvidenceReference` instance) can refer to content
/// extracted from a source (i.e., an "extracted" Subject) as information being used to derive the evidence expressed in this Subject.
///
/// # Examples
/// An application allows a researcher to extract information from a single census record about a person, representing the information as a persona
/// with an identifier "abcde". The researcher extracts additional information about the person from a birth certificate and the application assigns
/// the resulting persona an identifier "fghij". As the researcher gathers and analyzes the information, he will create a (working) `Person` conclusion.
/// When the researcher concludes that the person represented in "abcde" and in "fghij" are the same person, he will add two `EvidenceReference` instances
/// to the working `Person`: one for "abcde" and one for "fghij".
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct EvidenceReference {
    /// Reference to the supporting data.
    pub resource: Uri,

    /// The attribution of this evidence reference. If not provided, the attribution of the containing resource of the source reference is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
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

// Ideally we'd implement all the TryFroms with a blanket imple like impl <T: Subject> TryFrom<&T> for EvidenceReference.
// But that doesn't work due to https://github.com/rust-lang/rust/issues/50133. So insead we'll implement them with this macro.

try_from_evidencereference!(Person);
try_from_evidencereference!(Event);
try_from_evidencereference!(PlaceDescription);
try_from_evidencereference!(Relationship);

#[cfg(test)]
mod test {
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
        assert_eq!(evidence_reference, data.evidence_reference)
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
        )
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
}

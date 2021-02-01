use crate::{Attribution, Event, GedcomxError, Person, PlaceDescription, Relationship, Uri};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct EvidenceReference {
    pub resource: Uri,

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

    pub fn builder(resource: Uri) -> EvidenceReferenceBuilder {
        EvidenceReferenceBuilder::new(resource)
    }
}

pub struct EvidenceReferenceBuilder(EvidenceReference);

impl EvidenceReferenceBuilder {
    pub(crate) fn new(resource: Uri) -> Self {
        Self(EvidenceReference {
            resource,
            ..EvidenceReference::default()
        })
    }

    pub fn build(&self) -> EvidenceReference {
        EvidenceReference::new(self.0.resource.clone(), self.0.attribution.clone())
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
            EvidenceReference::builder(Uri::from("S-1")).build()
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
        let evidence_reference = EvidenceReference::builder(Uri::from("S-1")).build();

        let json = serde_json::to_string(&evidence_reference).unwrap();

        assert_eq!(json, r#"{"resource":"S-1"}"#);
    }
}

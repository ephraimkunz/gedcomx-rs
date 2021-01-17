use crate::components::{Attribution, Uri};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct EvidenceReference {
    pub resource: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
}

impl EvidenceReference {
    pub fn new(resource: Uri) -> Self {
        Self {
            resource,
            attribution: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::TestData;

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
        assert_eq!(
            evidence_reference,
            EvidenceReference {
                resource: Uri::from("S-1"),
                attribution: data.attribution(),
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{
            "resource" : "S-1"   
        }"#;

        let evidence_reference: EvidenceReference = serde_json::from_str(json).unwrap();
        assert_eq!(evidence_reference, EvidenceReference::new(Uri::from("S-1")))
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let evidence_reference = EvidenceReference {
            resource: Uri::from("S-1"),
            attribution: data.attribution(),
        };

        let json = serde_json::to_string(&evidence_reference).unwrap();

        assert_eq!(
            json,
            r#"{"resource":"S-1","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}"#
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let evidence_reference = EvidenceReference::new(Uri::from("S-1"));

        let json = serde_json::to_string(&evidence_reference).unwrap();

        assert_eq!(json, r#"{"resource":"S-1"}"#);
    }
}

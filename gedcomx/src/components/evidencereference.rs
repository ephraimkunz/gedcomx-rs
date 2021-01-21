use crate::{Attribution, GedcomxError, Person, Uri};
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

impl TryFrom<&Person> for EvidenceReference {
    type Error = GedcomxError;
    fn try_from(person: &Person) -> Result<Self, Self::Error> {
        match &person.subject.conclusion.id {
            Some(id) => Ok(Self::builder(format!("#{}", id).into()).build()),
            None => Err(GedcomxError::NoId("Person".to_string())),
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

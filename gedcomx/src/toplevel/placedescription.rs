use crate::{
    Conclusion, ConclusionData, Date, ResourceReference, Subject, SubjectData, TextValue, Uri,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlaceDescription {
    #[serde(flatten)]
    pub subject: SubjectData,

    pub names: Vec<TextValue>, // Must contain at least 1 name.

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub typee: Option<Uri>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_description: Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_description: Option<ResourceReference>,
}

impl Conclusion for PlaceDescription {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Subject for PlaceDescription {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

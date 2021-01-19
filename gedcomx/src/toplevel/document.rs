use crate::{Attribution, Conclusion, ConclusionData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(flatten)]
    pub conclusion: ConclusionData,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub document_type: Option<DocumentType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<Attribution>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum DocumentType {
    Analysis,
    Abstract,
    Transcription,
    Translation,
}

impl Conclusion for Document {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }
}

use crate::{Date, PlaceReference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Coverage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial: Option<PlaceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal: Option<Date>,
}

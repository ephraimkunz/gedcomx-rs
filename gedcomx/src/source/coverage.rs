use crate::{Date, PlaceReference};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Coverage {
    pub spatial: Option<PlaceReference>,

    pub temporal: Option<Date>,
}

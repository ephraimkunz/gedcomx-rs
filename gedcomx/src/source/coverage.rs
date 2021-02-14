use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Date, PlaceReference};

/// The coverage of a resource.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Coverage {
    /// The spatial (i.e., geographic) coverage.
    pub spatial: Option<PlaceReference>,

    /// The temporal coverage.
    pub temporal: Option<Date>,
}

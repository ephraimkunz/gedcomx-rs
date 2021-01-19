use crate::{Address, Id, Identifier, OnlineAccount, ResourceReference, TextValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct Agent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub identifiers: Vec<Identifier>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub names: Vec<TextValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub accounts: Vec<OnlineAccount>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub emails: Vec<ResourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub phones: Vec<ResourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub addresses: Vec<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<ResourceReference>,
}

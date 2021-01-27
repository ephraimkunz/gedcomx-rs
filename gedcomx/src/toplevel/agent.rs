use crate::{Address, Id, Identifier, OnlineAccount, ResourceReference, TextValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
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

impl Agent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Option<Id>,
        identifiers: Vec<Identifier>,
        names: Vec<TextValue>,
        homepage: Option<ResourceReference>,
        openid: Option<ResourceReference>,
        accounts: Vec<OnlineAccount>,
        emails: Vec<ResourceReference>,
        phones: Vec<ResourceReference>,
        addresses: Vec<Address>,
        person: Option<ResourceReference>,
    ) -> Self {
        Self {
            id,
            identifiers,
            names,
            homepage,
            openid,
            accounts,
            emails,
            phones,
            addresses,
            person,
        }
    }

    pub fn builder() -> AgentBuilder {
        AgentBuilder::new()
    }
}

pub struct AgentBuilder(Agent);

impl AgentBuilder {
    pub(crate) fn new() -> Self {
        Self(Agent { ..Agent::default() })
    }

    pub fn id<I: Into<Id>>(&mut self, id: I) -> &mut Self {
        self.0.id = Some(id.into());
        self
    }

    pub fn name<I: Into<TextValue>>(&mut self, name: I) -> &mut Self {
        self.0.names.push(name.into());
        self
    }

    pub fn address(&mut self, address: Address) -> &mut Self {
        self.0.addresses.push(address);
        self
    }

    pub fn email<I: Into<String>>(&mut self, email: I) -> &mut Self {
        self.0
            .emails
            .push(format!("mailto:{}", email.into()).into());
        self
    }

    pub fn build(&self) -> Agent {
        Agent::new(
            self.0.id.clone(),
            self.0.identifiers.clone(),
            self.0.names.clone(),
            self.0.homepage.clone(),
            self.0.openid.clone(),
            self.0.accounts.clone(),
            self.0.emails.clone(),
            self.0.phones.clone(),
            self.0.addresses.clone(),
            self.0.person.clone(),
        )
    }
}

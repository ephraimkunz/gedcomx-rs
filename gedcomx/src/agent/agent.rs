use crate::{Address, Id, Identifier, OnlineAccount, ResourceReference, TextValue};
use serde::{Deserialize, Serialize};

/// Someone or something that curates genealogical data, such as a genealogical researcher, user of software,
/// organization, or group.
///
/// In genealogical research, an agent often takes the role of a contributor.
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[non_exhaustive]
pub struct Agent {
    /// An identifier for the data structure holding the agent data.
    /// The id is to be used as a "fragment identifier" as defined by [RFC 3986, Section 3.5](https://tools.ietf.org/html/rfc3986#section-3.5).
    /// As such, the constraints of the id are provided in the definition of the media type (e.g. XML, JSON) of the data structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,

    /// A list of identifiers for the agent.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub identifiers: Vec<Identifier>, // TODO: Write custom serializer / deserializer for this. Current behavior doesn't match spec.

    /// The name(s) of the person or organization. If more than one name is provided, names are assumed to be given
    /// in order of preference, with the most preferred name in the first position in the list.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub names: Vec<TextValue>,

    /// The homepage of the person or organization. Note this is different from the homepage of the service where the
    /// person or organization has an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<ResourceReference>,

    /// The [openid](https://openid.net) of the person or organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<ResourceReference>,

    /// The online account(s) of the person or organization.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub accounts: Vec<OnlineAccount>,

    /// The email address(es) of the person or organization. If provided, MUST resolve to a valid e-mail address (e.g. "mailto:someone@gedcomx.org").
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub emails: Vec<ResourceReference>, // TODO: Should I use a type here that would validate this is a valid email address?

    /// The phone(s) (voice, fax, mobile) of the person or organization. If provided, MUST resolve to a valid phone number (e.g. "tel:+1-201-555-0123").
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub phones: Vec<ResourceReference>, // TODO: Should I use a type that would validate this is a valid phone number?

    /// The address(es) of the person or organization.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub addresses: Vec<Address>,

    /// A reference to the person that describes this agent. MUST resolve to an instance of [Person](crate::Person).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<ResourceReference>, // TODO: Enforce constraint?
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

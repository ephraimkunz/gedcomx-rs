use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{Address, Id, Identifier, OnlineAccount, Person, ResourceReference, Result, TextValue};

/// Someone or something that curates genealogical data, such as a genealogical
/// researcher, user of software, organization, or group.
///
/// In genealogical research, an agent often takes the role of a contributor.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
#[yaserde(rename = "agent")]
#[non_exhaustive]
pub struct Agent {
    /// An identifier for the data structure holding the agent data.
    /// The id is to be used as a "fragment identifier" as defined by [RFC 3986, Section 3.5](https://tools.ietf.org/html/rfc3986#section-3.5).
    /// As such, the constraints of the id are provided in the definition of the
    /// media type (e.g. XML, JSON) of the data structure.
    #[yaserde(attribute)]
    pub id: Option<Id>,

    /// A list of identifiers for the agent.
    #[yaserde(rename = "identifier")]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        with = "crate::serde_vec_identifier_to_map"
    )]
    pub identifiers: Vec<Identifier>,

    /// The name(s) of the person or organization. If more than one name is
    /// provided, names are assumed to be given in order of preference, with
    /// the most preferred name in the first position in the list.
    #[yaserde(rename = "name")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub names: Vec<TextValue>,

    /// The homepage of the person or organization. Note this is different from
    /// the homepage of the service where the person or organization has an
    /// account.
    pub homepage: Option<ResourceReference>,

    /// The [openid](https://openid.net) of the person or organization.
    pub openid: Option<ResourceReference>,

    /// The online account(s) of the person or organization.
    #[yaserde(rename = "account")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub accounts: Vec<OnlineAccount>,

    /// The email address(es) of the person or organization. If provided, MUST
    /// resolve to a valid e-mail address (e.g. "mailto:someone@gedcomx.org").
    #[yaserde(rename = "email")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub emails: Vec<ResourceReference>, /* TODO: Should I use a type here that would validate
                                         * this is a valid email address? */

    /// The phone(s) (voice, fax, mobile) of the person or organization. If
    /// provided, MUST resolve to a valid phone number (e.g.
    /// "tel:+1-201-555-0123").
    #[yaserde(rename = "phone")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub phones: Vec<ResourceReference>, /* TODO: Should I use a type that would validate this is
                                         * a valid phone number? */

    /// The address(es) of the person or organization.
    #[yaserde(rename = "address")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub addresses: Vec<Address>,

    /// A reference to the person that describes this agent. MUST resolve to an
    /// instance of [Person](crate::Person).
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
        Self(Agent::default())
    }

    pub fn id<I: Into<Id>>(&mut self, id: I) -> &mut Self {
        self.0.id = Some(id.into());
        self
    }

    pub fn identifier(&mut self, identifier: Identifier) -> &mut Self {
        self.0.identifiers.push(identifier);
        self
    }

    pub fn name<I: Into<TextValue>>(&mut self, name: I) -> &mut Self {
        self.0.names.push(name.into());
        self
    }

    pub fn homepage<I: Into<ResourceReference>>(&mut self, homepage: I) -> &mut Self {
        self.0.homepage = Some(homepage.into());
        self
    }

    pub fn openid<I: Into<ResourceReference>>(&mut self, openid: I) -> &mut Self {
        self.0.openid = Some(openid.into());
        self
    }

    pub fn account(&mut self, account: OnlineAccount) -> &mut Self {
        self.0.accounts.push(account);
        self
    }

    pub fn email<I: Into<ResourceReference>>(&mut self, email: I) -> &mut Self {
        self.0.emails.push(email.into());
        self
    }

    pub fn phone<I: Into<ResourceReference>>(&mut self, phone: I) -> &mut Self {
        self.0.phones.push(phone.into());
        self
    }

    pub fn address(&mut self, address: Address) -> &mut Self {
        self.0.addresses.push(address);
        self
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `person` has no `id` set.
    pub fn person(&mut self, person: &Person) -> Result<&mut Self> {
        self.0.person = Some(person.try_into()?);
        Ok(self)
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

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::IdentifierType;

    #[test]
    fn xml_deserialize() {
        let xml = r##"
        <agent id="local_id">
            <identifier type="http://gedcomx.org/Primary">primaryIdentifier</identifier>
            <name>Ephraim Kunz</name>
            <name lang="es">Ephraim Kunz Spanish</name>
            <homepage resource="www.ephraimkunz.com"/>
            <openid resource="some_openid_value"/>
            <account>
                <serviceHomepage resource="http://familysearch.org/"/>
                <accountName>Family Search Account</accountName>
            </account>
            <email resource="mailto:someone@gedcomx.org"/>
            <email resource="mailto:someone2@gedcomx.org"/>
            <phone resource="tel:+1-201-555-0123"/>
            <address>
                <country>United States</country>
            </address>
            <person resource="#P-1"/>    
        </agent>"##;

        let person = Person::builder().id("P-1").build();

        let expected_agent = Agent::builder()
            .id("local_id")
            .identifier(Identifier::new(
                "primaryIdentifier",
                Some(IdentifierType::Primary),
            ))
            .name("Ephraim Kunz")
            .name(TextValue::new("Ephraim Kunz Spanish", Some("es")))
            .homepage("www.ephraimkunz.com")
            .openid("some_openid_value")
            .account(OnlineAccount::new(
                "http://familysearch.org/",
                "Family Search Account",
            ))
            .email("mailto:someone@gedcomx.org")
            .email("mailto:someone2@gedcomx.org")
            .phone("tel:+1-201-555-0123")
            .address(Address::builder().country("United States").build())
            .person(&person)
            .unwrap()
            .build();
        let agent: Agent = yaserde::de::from_str(xml).unwrap();

        assert_eq!(agent, expected_agent)
    }

    #[test]
    fn xml_serialize() {
        let person = Person::builder().id("P-1").build();
        let agent = Agent::builder()
            .id("local_id")
            .identifier(Identifier::new(
                "primaryIdentifier",
                Some(IdentifierType::Primary),
            ))
            .name("Ephraim Kunz")
            .name(TextValue::new("Ephraim Kunz Spanish", Some("es")))
            .homepage("www.ephraimkunz.com")
            .openid("some_openid_value")
            .account(OnlineAccount::new(
                "http://familysearch.org/",
                "Family Search Account",
            ))
            .email("mailto:someone@gedcomx.org")
            .email("mailto:someone2@gedcomx.org")
            .phone("tel:+1-201-555-0123")
            .address(Address::builder().country("United States").build())
            .person(&person)
            .unwrap()
            .build();

        let xml = yaserde::ser::to_string_content(&agent).unwrap();
        let expected_xml = r##"<identifier type="http://gedcomx.org/Primary">primaryIdentifier</identifier><name>Ephraim Kunz</name><name lang="es">Ephraim Kunz Spanish</name><homepage resource="www.ephraimkunz.com" /><openid resource="some_openid_value" /><account><serviceHomepage resource="http://familysearch.org/" /><accountName>Family Search Account</accountName></account><email resource="mailto:someone@gedcomx.org" /><email resource="mailto:someone2@gedcomx.org" /><phone resource="tel:+1-201-555-0123" /><address><country>United States</country></address><person resource="#P-1" />"##;
        assert_eq!(xml, expected_xml)
    }
}

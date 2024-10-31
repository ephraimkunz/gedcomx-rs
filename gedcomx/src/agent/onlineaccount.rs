use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::ResourceReference;

/// A description of an account for an online service provider.
#[derive(
    Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default, Eq,
)]
#[yaserde(
    rename = "account",
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OnlineAccount {
    /// The homepage of the service that provides this account.
    #[yaserde(rename = "serviceHomepage", prefix = "gx")]
    pub service_homepage: ResourceReference,

    /// The name, label, or id that uniquely identifies the account maintained
    /// by the online service provider.
    #[yaserde(rename = "accountName", prefix = "gx")]
    pub account_name: String,
}

impl OnlineAccount {
    pub fn new<I, J>(service_homepage: I, account_name: J) -> Self
    where
        I: Into<ResourceReference>,
        J: Into<String>,
    {
        Self {
            service_homepage: service_homepage.into(),
            account_name: account_name.into(),
        }
    }
}

impl Arbitrary for OnlineAccount {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(ResourceReference::arbitrary(g), crate::arbitrary_trimmed(g))
    }
}

#[cfg(test)]
mod test {
    use yaserde::ser::Config;

    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "serviceHomepage" : {
              "resource" : "http://familysearch.org/"
            },
            "accountName" : "Family Search Account"          
        }"#;

        let online_account: OnlineAccount = serde_json::from_str(json).unwrap();
        assert_eq!(
            online_account,
            OnlineAccount {
                service_homepage: ResourceReference::from("http://familysearch.org/"),
                account_name: "Family Search Account".to_string(),
            }
        );
    }

    #[test]
    fn json_serialize() {
        let online_account = OnlineAccount {
            service_homepage: ResourceReference::from("http://familysearch.org/"),
            account_name: "Family Search Account".to_string(),
        };

        let json = serde_json::to_string(&online_account).unwrap();

        assert_eq!(
            json,
            r#"{"serviceHomepage":{"resource":"http://familysearch.org/"},"accountName":"Family Search Account"}"#
        );
    }

    #[test]
    fn xml_deserialize() {
        let xml = r#"<account xmlns="http://gedcomx.org/v1/"><serviceHomepage resource="http://familysearch.org/"/>
        <accountName>Family Search Account</accountName></account>"#;

        let online_account: OnlineAccount = yaserde::de::from_str(xml).unwrap();
        assert_eq!(
            online_account,
            OnlineAccount {
                service_homepage: ResourceReference::from("http://familysearch.org/"),
                account_name: "Family Search Account".to_string(),
            }
        );
    }

    #[test]
    fn xml_serialize() {
        let online_account = OnlineAccount {
            service_homepage: ResourceReference::from("http://familysearch.org/"),
            account_name: "Family Search Account".to_string(),
        };

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };
        let xml = yaserde::ser::to_string_with_config(&online_account, &config).unwrap();

        assert_eq!(
            xml,
            r#"<account xmlns="http://gedcomx.org/v1/"><serviceHomepage resource="http://familysearch.org/" /><accountName>Family Search Account</accountName></account>"#
        );
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: OnlineAccount) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: OnlineAccount = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: OnlineAccount) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: OnlineAccount = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

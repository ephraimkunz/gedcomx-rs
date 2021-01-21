use crate::components::ResourceReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OnlineAccount {
    pub service_homepage: ResourceReference,
    pub account_name: String,
}

impl OnlineAccount {
    pub fn new(service_homepage: ResourceReference, account_name: String) -> Self {
        Self {
            service_homepage,
            account_name,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

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
        )
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
        )
    }
}

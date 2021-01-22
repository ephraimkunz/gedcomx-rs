use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub street3: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub street4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub street5: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub street6: Option<String>,
}

impl Address {
    pub fn new() -> Self {
        Self {
            value: None,
            city: None,
            country: None,
            postal_code: None,
            state_or_province: None,
            street: None,
            street2: None,
            street3: None,
            street4: None,
            street5: None,
            street6: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "value" : "value",
            "city" : "city",
            "country" : "country",
            "postalCode" : "postalcode",
            "stateOrProvince" : "stateorprovince",
            "street" : "street",
            "street2" : "street2",
            "street3" : "street3",
            "street4" : "street4",
            "street5" : "street5",
            "street6" : "street6"          
        }"#;

        let address: Address = serde_json::from_str(json).unwrap();
        assert_eq!(
            address,
            Address {
                value: Some("value".to_string()),
                city: Some("city".to_string()),
                country: Some("country".to_string()),
                postal_code: Some("postalcode".to_string()),
                state_or_province: Some("stateorprovince".to_string()),
                street: Some("street".to_string()),
                street2: Some("street2".to_string()),
                street3: Some("street3".to_string()),
                street4: Some("street4".to_string()),
                street5: Some("street5".to_string()),
                street6: Some("street6".to_string()),
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{}"#;

        let address: Address = serde_json::from_str(json).unwrap();
        assert_eq!(address, Address::new())
    }

    #[test]
    fn json_serialize() {
        let address = Address {
            value: Some("value".to_string()),
            city: Some("city".to_string()),
            country: Some("country".to_string()),
            postal_code: Some("postalcode".to_string()),
            state_or_province: Some("stateorprovince".to_string()),
            street: Some("street".to_string()),
            street2: Some("street2".to_string()),
            street3: Some("street3".to_string()),
            street4: Some("street4".to_string()),
            street5: Some("street5".to_string()),
            street6: Some("street6".to_string()),
        };

        let json = serde_json::to_string(&address).unwrap();

        assert_eq!(
            json,
            r#"{"value":"value","city":"city","country":"country","postalCode":"postalcode","stateOrProvince":"stateorprovince","street":"street","street2":"street2","street3":"street3","street4":"street4","street5":"street5","street6":"street6"}"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let address = Address::new();

        let json = serde_json::to_string(&address).unwrap();

        assert_eq!(json, r#"{}"#)
    }
}

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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        value: Option<String>,
        city: Option<String>,
        country: Option<String>,
        postal_code: Option<String>,
        state_or_province: Option<String>,
        street: Option<String>,
        street2: Option<String>,
        street3: Option<String>,
        street4: Option<String>,
        street5: Option<String>,
        street6: Option<String>,
    ) -> Self {
        Self {
            value,
            city,
            country,
            postal_code,
            state_or_province,
            street,
            street2,
            street3,
            street4,
            street5,
            street6,
        }
    }

    pub fn builder() -> AddressBuilder {
        AddressBuilder::new()
    }
}

pub struct AddressBuilder(Address);

impl AddressBuilder {
    pub(crate) fn new() -> Self {
        Self(Address::default())
    }

    pub fn value<I: Into<String>>(&mut self, value: I) -> &mut Self {
        self.0.value = Some(value.into());
        self
    }

    pub fn city<I: Into<String>>(&mut self, city: I) -> &mut Self {
        self.0.city = Some(city.into());
        self
    }

    pub fn country<I: Into<String>>(&mut self, country: I) -> &mut Self {
        self.0.country = Some(country.into());
        self
    }

    pub fn postal_code<I: Into<String>>(&mut self, postal_code: I) -> &mut Self {
        self.0.postal_code = Some(postal_code.into());
        self
    }

    pub fn state_or_province<I: Into<String>>(&mut self, state_or_province: I) -> &mut Self {
        self.0.state_or_province = Some(state_or_province.into());
        self
    }

    pub fn street<I: Into<String>>(&mut self, street: I) -> &mut Self {
        self.0.street = Some(street.into());
        self
    }

    pub fn street2<I: Into<String>>(&mut self, street2: I) -> &mut Self {
        self.0.street2 = Some(street2.into());
        self
    }

    pub fn street3<I: Into<String>>(&mut self, street3: I) -> &mut Self {
        self.0.street3 = Some(street3.into());
        self
    }

    pub fn street4<I: Into<String>>(&mut self, street4: I) -> &mut Self {
        self.0.street4 = Some(street4.into());
        self
    }

    pub fn street5<I: Into<String>>(&mut self, street5: I) -> &mut Self {
        self.0.street5 = Some(street5.into());
        self
    }

    pub fn street6<I: Into<String>>(&mut self, street6: I) -> &mut Self {
        self.0.street6 = Some(street6.into());
        self
    }

    pub fn build(&self) -> Address {
        Address::new(
            self.0.value.clone(),
            self.0.city.clone(),
            self.0.country.clone(),
            self.0.postal_code.clone(),
            self.0.state_or_province.clone(),
            self.0.street.clone(),
            self.0.street2.clone(),
            self.0.street3.clone(),
            self.0.street4.clone(),
            self.0.street5.clone(),
            self.0.street6.clone(),
        )
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
        assert_eq!(address, Address::default())
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
        let address = Address::default();

        let json = serde_json::to_string(&address).unwrap();

        assert_eq!(json, r#"{}"#)
    }
}

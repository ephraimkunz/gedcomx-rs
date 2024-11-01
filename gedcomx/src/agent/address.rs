use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

/// A street or postal address of a person or organization.
#[skip_serializing_none]
#[derive(
    Debug, Serialize, YaSerialize, YaDeserialize, Deserialize, PartialEq, Eq, Clone, Default,
)]
#[yaserde(
    rename = "address",
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Address {
    /// A full representation of the complete address.
    #[yaserde(prefix = "gx")]
    pub value: Option<String>,

    /// The city.
    #[yaserde(prefix = "gx")]
    pub city: Option<String>,

    /// The country.
    #[yaserde(prefix = "gx")]
    pub country: Option<String>,

    /// The postal code.
    #[yaserde(rename = "postalCode", prefix = "gx")]
    pub postal_code: Option<String>,

    /// The state or province.
    #[yaserde(rename = "stateOrProvince", prefix = "gx")]
    pub state_or_province: Option<String>,

    /// The street.
    #[yaserde(prefix = "gx")]
    pub street: Option<String>,

    /// The street (second line).
    #[yaserde(prefix = "gx")]
    pub street2: Option<String>,

    /// The street (third line).
    #[yaserde(prefix = "gx")]
    pub street3: Option<String>,

    /// The street (fourth line).
    #[yaserde(prefix = "gx")]
    pub street4: Option<String>,

    /// The street (fifth line).
    #[yaserde(prefix = "gx")]
    pub street5: Option<String>,

    /// The street (sixth line).
    #[yaserde(prefix = "gx")]
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

impl Arbitrary for Address {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::builder()
            .value(crate::arbitrary_trimmed(g))
            .city(crate::arbitrary_trimmed(g))
            .country(crate::arbitrary_trimmed(g))
            .postal_code(crate::arbitrary_trimmed(g))
            .state_or_province(crate::arbitrary_trimmed(g))
            .street(crate::arbitrary_trimmed(g))
            .street2(crate::arbitrary_trimmed(g))
            .street3(crate::arbitrary_trimmed(g))
            .street4(crate::arbitrary_trimmed(g))
            .street5(crate::arbitrary_trimmed(g))
            .street6(crate::arbitrary_trimmed(g))
            .build()
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
    use pretty_assertions::assert_eq;
    use yaserde::ser::Config;

    use super::*;

    #[test]
    fn builder() {
        let address_1 = Address {
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

        let address_2 = Address::builder()
            .value("value")
            .city("city")
            .country("country")
            .postal_code("postalcode")
            .state_or_province("stateorprovince")
            .street("street")
            .street2("street2")
            .street3("street3")
            .street4("street4")
            .street5("street5")
            .street6("street6")
            .build();

        assert_eq!(address_1, address_2);
    }

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
        );
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{}"#;

        let address: Address = serde_json::from_str(json).unwrap();
        assert_eq!(address, Address::default());
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
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let address = Address::default();

        let json = serde_json::to_string(&address).unwrap();

        assert_eq!(json, r#"{}"#);
    }

    #[test]
    fn xml_deserialize() {
        let xml = r#"<address xmlns="http://gedcomx.org/v1/">
            <city>East Palo Alto</city>
            <country>United States</country>
            <postalCode>94303</postalCode>
            <stateOrProvince>California</stateOrProvince>
            <street>2299 Poplar Ave</street>
        </address>"#;

        let expected_address = Address::builder()
            .city("East Palo Alto")
            .country("United States")
            .postal_code("94303")
            .state_or_province("California")
            .street("2299 Poplar Ave")
            .build();
        let address: Address = yaserde::de::from_str(xml).unwrap();

        assert_eq!(address, expected_address);
    }

    #[test]
    fn xml_serialize() {
        let address = Address::builder()
            .city("East Palo Alto")
            .country("United States")
            .postal_code("94303")
            .state_or_province("California")
            .street("2299 Poplar Ave")
            .build();

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };
        let xml = yaserde::ser::to_string_with_config(&address, &config).unwrap();

        assert_eq!(
            xml,
            r#"<address xmlns="http://gedcomx.org/v1/"><city>East Palo Alto</city><country>United States</country><postalCode>94303</postalCode><stateOrProvince>California</stateOrProvince><street>2299 Poplar Ave</street></address>"#
        );
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: Address) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: Address = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: Address) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: Address = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

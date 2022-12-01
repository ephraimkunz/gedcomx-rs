use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::Uri;

/// Used to supply additional details, annotations, tags, or other qualifying
/// data to a specific data element.
#[skip_serializing_none]
#[derive(
    Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default, Eq,
)]
#[non_exhaustive]
pub struct Qualifier {
    /// The name of the qualifier. The name should be an element of a
    /// constrained vocabulary and is used to determine meaning of the
    /// qualifier.
    #[yaserde(attribute)]
    pub name: Uri,

    /// The value of the qualifier. Some qualifiers may not have values,
    /// indicating that the qualifier is to be treated more like a "tag".
    #[yaserde(text)]
    pub value: Option<String>,
}

impl Qualifier {
    pub fn new<U: Into<Uri>, S: Into<String>>(name: U, value: Option<S>) -> Self {
        Self {
            name: name.into(),
            value: value.map(std::convert::Into::into),
        }
    }
}

impl Arbitrary for Qualifier {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(
            crate::arbitrary_trimmed(g),
            Some(crate::arbitrary_trimmed(g)),
        )
    }
}

#[cfg(test)]
mod test {
    use yaserde::ser::Config;

    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r##"{
            "name" : "http://gedcomx.org/QualifierName",
            "value" : "..."
          }"##;

        let expected = Qualifier::new("http://gedcomx.org/QualifierName", Some("..."));
        let qualifier: Qualifier = serde_json::from_str(json).unwrap();
        assert_eq!(qualifier, expected);
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r##"{
            "name" : "http://gedcomx.org/QualifierName"
        }"##;

        let expected = Qualifier::new("http://gedcomx.org/QualifierName", None::<String>);
        let qualifier: Qualifier = serde_json::from_str(json).unwrap();
        assert_eq!(qualifier, expected);
    }

    #[test]
    fn json_serialize() {
        let qualifier = Qualifier::new("http://gedcomx.org/QualifierName", Some("..."));
        let json = serde_json::to_string(&qualifier).unwrap();
        let expected = r##"{"name":"http://gedcomx.org/QualifierName","value":"..."}"##;

        assert_eq!(json, expected);
    }

    #[test]
    fn json_serialize_optional_fields() {
        let qualifier = Qualifier::new("http://gedcomx.org/QualifierName", None::<String>);
        let json = serde_json::to_string(&qualifier).unwrap();
        let expected = r##"{"name":"http://gedcomx.org/QualifierName"}"##;

        assert_eq!(json, expected);
    }

    #[test]
    fn xml_serialize() {
        let qualifier = Qualifier::new(
            "http://gedcomx.org/QualifierName",
            Some("...qualifier value..."),
        );

        let config = Config {
            write_document_declaration: false,
            ..Default::default()
        };

        let xml = yaserde::ser::to_string_with_config(&qualifier, &config).unwrap();

        let expected = r##"<Qualifier name="http://gedcomx.org/QualifierName">...qualifier value...</Qualifier>"##;

        assert_eq!(xml, expected);
    }

    #[test]
    fn xml_deserialize() {
        let xml = r##"<Qualifier name="http://gedcomx.org/QualifierName">...qualifier value...</Qualifier>"##;

        let qualifier: Qualifier = yaserde::de::from_str(xml).unwrap();
        let expected = Qualifier::new(
            "http://gedcomx.org/QualifierName",
            Some("...qualifier value..."),
        );
        assert_eq!(qualifier, expected);
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: Qualifier) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: Qualifier = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: Qualifier) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: Qualifier = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

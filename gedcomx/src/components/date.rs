use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Date {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub formal: Option<String>,
}

impl Date {
    pub fn new(original: Option<String>, formal: Option<String>) -> Self {
        Self { original, formal }
    }

    pub fn builder() -> DateBuilder {
        DateBuilder::new()
    }
}

pub struct DateBuilder(Date);

impl DateBuilder {
    pub(crate) fn new() -> Self {
        Self(Date::default())
    }

    pub fn original<I: Into<String>>(&mut self, original: I) -> &mut Self {
        self.0.original = Some(original.into());
        self
    }

    pub fn build(&self) -> Date {
        Date::new(self.0.original.clone(), self.0.formal.clone())
    }
}

// TODO: Implement custom serializer / deserializer on top of gedcomx_date library?
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DateX;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{
            "original" : "the original text"
          }"#;

        let date: Date = serde_json::from_str(json).unwrap();

        assert_eq!(
            date,
            Date {
                original: Some("the original text".to_string()),
                formal: None // Replace with formal date once we implement that.
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let json = r#"{}"#;

        let date: Date = serde_json::from_str(json).unwrap();

        assert_eq!(
            date,
            Date {
                original: None,
                formal: None
            }
        )
    }

    #[test]
    fn json_serialize() {
        let date = Date {
            original: Some("the original text".to_string()),
            formal: None, // Replace with formal date once we implement that.
        };

        let json = serde_json::to_string(&date).unwrap();

        assert_eq!(json, r#"{"original":"the original text"}"#)
    }

    #[test]
    fn json_serialize_optional_fields() {
        let date = Date {
            original: None,
            formal: None,
        };

        let json = serde_json::to_string(&date).unwrap();

        assert_eq!(json, r#"{}"#)
    }
}

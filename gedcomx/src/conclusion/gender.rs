use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{Conclusion, ConclusionData, EnumAsString, Uri};

/// A gender of a person.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Gender {
    /// The type of the gender.
    #[serde(rename = "type")]
    pub gender_type: GenderType,

    #[serde(flatten)]
    pub conclusion: ConclusionData,
}

impl Gender {
    pub fn new(conclusion: ConclusionData, gender_type: GenderType) -> Self {
        Self {
            conclusion,
            gender_type,
        }
    }

    pub fn builder(gender_type: GenderType) -> GenderBuilder {
        GenderBuilder::new(gender_type)
    }
}

pub struct GenderBuilder(Gender);

impl GenderBuilder {
    conclusion_builder_functions!(Gender);

    pub(crate) fn new(gender_type: GenderType) -> Self {
        Self(Gender {
            gender_type,
            ..Gender::default()
        })
    }

    #[allow(dead_code)] // Nothing in the crate currently uses it, but clients of the crate may want it.
    fn build(&self) -> Gender {
        Gender {
            gender_type: self.0.gender_type.clone(),
            conclusion: self.0.conclusion.clone(),
        }
    }
}

impl From<GenderType> for Gender {
    fn from(gender_type: GenderType) -> Self {
        Self {
            gender_type,
            ..Self::default()
        }
    }
}

impl Conclusion for Gender {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("Gender")
    }
}

/// Type of gender.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum GenderType {
    /// Male gender.
    Male,

    /// Female gender.
    Female,

    /// Unknown gender.
    Unknown,

    /// Intersex (assignment at birth).
    Intersex,

    Custom(Uri),
}

impl Default for GenderType {
    fn default() -> Self {
        Self::Custom(Uri::from(String::default()))
    }
}

impl From<EnumAsString> for GenderType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Male" => Self::Male,
            "http://gedcomx.org/Female" => Self::Female,
            "http://gedcomx.org/Unknown" => Self::Unknown,
            "http://gedcomx.org/Intersex" => Self::Intersex,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for GenderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Male => write!(f, "http://gedcomx.org/Male"),
            Self::Female => write!(f, "http://gedcomx.org/Female"),
            Self::Unknown => write!(f, "http://gedcomx.org/Unknown"),
            Self::Intersex => write!(f, "http://gedcomx.org/Intersex"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestData;

    #[test]
    fn json_deserialize() {
        let data = TestData::new();

        let json = r#"{
            "type" : "http://gedcomx.org/Male",
            "id" : "local_id",
            "lang" : "en",
            "sources" : [ {
                "description" : "SD-1",
                "descriptionId" : "Description id of the target source",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                },
                "qualifiers" : [ { "name" : "http://gedcomx.org/RectangleRegion", "value" : "rectangle region value" } ]          
            }],
            "analysis" : {
              "resource" : "http://identifier/for/analysis/document"
            },
            "notes" : [ {
                "lang" : "en",
                "subject" : "subject",
                "text" : "This is a note",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                }        
            } ],
            "confidence" : "http://gedcomx.org/High",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            }  
        }"#;

        let gender: Gender = serde_json::from_str(json).unwrap();

        assert_eq!(
            gender,
            Gender {
                gender_type: GenderType::Male,
                conclusion: data.conclusion_data
            }
        )
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let gender = Gender {
            gender_type: GenderType::Male,
            conclusion: data.conclusion_data,
        };

        let json = serde_json::to_string(&gender).unwrap();

        assert_eq!(
            json,
            r#"{"type":"http://gedcomx.org/Male","id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}"#
        )
    }
}

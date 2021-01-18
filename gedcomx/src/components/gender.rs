use crate::components::{Conclusion, ConclusionData, Id, Identifiable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct Gender {
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum GenderType {
    #[serde(rename = "http://gedcomx.org/Male")]
    Male,

    #[serde(rename = "http://gedcomx.org/Female")]
    Female,

    #[serde(rename = "http://gedcomx.org/Unknown")]
    Unknown,

    #[serde(rename = "http://gedcomx.org/Intersex")]
    Intersex,
}

impl Conclusion for Gender {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }
}

impl Identifiable for Gender {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::TestData;

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

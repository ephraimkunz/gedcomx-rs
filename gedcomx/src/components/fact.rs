use crate::{
    components::{Conclusion, ConclusionData, Date, Id, Identifiable, PlaceReference},
    Qualifier,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct Fact {
    #[serde(rename = "type")]
    pub fact_type: FactType,

    #[serde(flatten)]
    pub conclusion: ConclusionData,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<PlaceReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub qualifiers: Vec<Qualifier>,
}

impl Conclusion for Fact {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }
}

impl Identifiable for Fact {
    fn id(&self) -> &Id {
        &self.conclusion().id
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum FactType {
    #[serde(rename = "http://gedcomx.org/Adoption")]
    Adoption,

    #[serde(rename = "http://gedcomx.org/Birth")]
    Birth,

    #[serde(rename = "http://gedcomx.org/Burial")]
    Burial,

    #[serde(rename = "http://gedcomx.org/Christening")]
    Christening,

    #[serde(rename = "http://gedcomx.org/Death")]
    Death,

    #[serde(rename = "http://gedcomx.org/Residence")]
    Residence,

    #[serde(rename = "http://gedcomx.org/Divorce")]
    Divorce,

    #[serde(rename = "http://gedcomx.org/Marriage")]
    Marriage,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum FactQualifier {
    Age,
    Cause,
    Religion,
    Transport,
    NonConsensual,
}

impl fmt::Display for FactQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "http://gedcomx.org/{:?}", self)
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
            "type" : "http://gedcomx.org/Birth",
            "place" : {
                "original" : "This is a place reference",
                "description" : "D-1"            
            },
            "value" : "the original value of the fact",
            "qualifiers" : [ { "name" : "http://gedcomx.org/Age", "value" : "val" } ],

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

        let fact: Fact = serde_json::from_str(json).unwrap();

        assert_eq!(
            fact,
            Fact {
                conclusion: data.conclusion_data,
                fact_type: FactType::Birth,
                place: Some(PlaceReference {
                    original: Some("This is a place reference".to_string()),
                    description_ref: Some("D-1".into())
                }),
                value: Some("the original value of the fact".to_string()),
                qualifiers: vec![Qualifier {
                    name: FactQualifier::Age.into(),
                    value: Some("val".to_string())
                }],
                date: None, // TODO: Add in once we get the date type working
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let data = TestData::new();

        let json = r#"{              
            "type" : "http://gedcomx.org/Birth",
    
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

        let fact: Fact = serde_json::from_str(json).unwrap();

        assert_eq!(
            fact,
            Fact {
                conclusion: data.conclusion_data,
                fact_type: FactType::Birth,
                place: None,
                value: None,
                qualifiers: vec![],
                date: None
            }
        )
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let fact = Fact {
            conclusion: data.conclusion_data,
            fact_type: FactType::Birth,
            place: Some(PlaceReference {
                original: Some("This is a place reference".to_string()),
                description_ref: Some("D-1".into()),
            }),
            value: Some("the original value of the fact".to_string()),
            qualifiers: vec![Qualifier {
                name: FactQualifier::Age.into(),
                value: Some("val".to_string()),
            }],
            date: None, // TODO: Add in once we get the date type working
        };

        let json = serde_json::to_string(&fact).unwrap();

        assert_eq!(
            json,
            r#"{"type":"http://gedcomx.org/Birth","id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"place":{"original":"This is a place reference","description":"D-1"},"value":"the original value of the fact","qualifiers":[{"name":"http://gedcomx.org/Age","value":"val"}]}"#
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let data = TestData::new();

        let fact = Fact {
            conclusion: data.conclusion_data,
            fact_type: FactType::Birth,
            place: None,
            value: None,
            qualifiers: vec![],
            date: None,
        };

        let json = serde_json::to_string(&fact).unwrap();

        assert_eq!(
            json,
            r#"{"type":"http://gedcomx.org/Birth","id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}"#
        );
    }
}

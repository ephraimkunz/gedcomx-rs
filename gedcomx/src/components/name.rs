use crate::{
    components::{Conclusion, ConclusionData, Date, Lang},
    Qualifier,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Name {
    #[serde(flatten)]
    pub conclusion: ConclusionData,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub name_type: Option<NameType>,

    pub name_forms: Vec<NameForm>, // Must be non-empty.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
}

impl Name {
    pub fn new(conclusion: ConclusionData, name_forms: Vec<NameForm>) -> Self {
        Self {
            conclusion,
            name_forms,
            name_type: None,
            date: None,
        }
    }
}

impl Conclusion for Name {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum NameType {
    #[serde(rename = "http://gedcomx.org/BirthName")]
    BirthName,

    #[serde(rename = "http://gedcomx.org/MarriedName")]
    MarriedName,

    #[serde(rename = "http://gedcomx.org/AlsoKnownAs")]
    AlsoKnownAs,

    #[serde(rename = "http://gedcomx.org/Nickname")]
    Nickname,

    #[serde(rename = "http://gedcomx.org/AdoptiveName")]
    AdoptiveName,

    #[serde(rename = "http://gedcomx.org/FormalName")]
    FormalName,

    #[serde(rename = "http://gedcomx.org/ReligiousName")]
    ReligiousName,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct NameForm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Lang>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_text: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parts: Vec<NamePart>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct NamePart {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub part_type: Option<NamePartType>,

    pub value: String,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub qualifiers: Vec<Qualifier>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum NamePartType {
    #[serde(rename = "http://gedcomx.org/Prefix")]
    Prefix,

    #[serde(rename = "http://gedcomx.org/Suffix")]
    Suffix,

    #[serde(rename = "http://gedcomx.org/Given")]
    Given,

    #[serde(rename = "http://gedcomx.org/Surname")]
    Surname,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum NamePartQualifier {
    Title,
    Primary,
    Secondary,
    Middle,
    Familiar,
    Religious,
    Family,
    Maiden,
    Patronymic,
    Matronymic,
    Geographic,
    Occupational,
    Characteristic,
    Postnom,
    Particle,
    RootName,
}

impl fmt::Display for NamePartQualifier {
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
            "type" : "http://gedcomx.org/BirthName",
            "nameForms" : [ {
                "lang" : "en",
                "fullText" : "full text of the name form",
                "parts" : [ {
                    "type" : "http://gedcomx.org/Surname",
                    "value" : "value of the name part",
                    "qualifiers" : [ { "name" : "http://gedcomx.org/Family" }, { "name" : "http://gedcomx.org/Patronymic" } ]                  
                }]                
            }],

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

        let name: Name = serde_json::from_str(json).unwrap();

        assert_eq!(
            name,
            Name {
                conclusion: data.conclusion_data,
                name_type: Some(NameType::BirthName),
                date: None, // TODO: Add in once we get the date type working
                name_forms: vec![NameForm {
                    lang: Some("en".to_string()),
                    full_text: Some("full text of the name form".to_string()),
                    parts: vec![NamePart {
                        part_type: Some(NamePartType::Surname),
                        value: "value of the name part".to_string(),
                        qualifiers: vec![
                            Qualifier {
                                name: NamePartQualifier::Family.into(),
                                value: None
                            },
                            Qualifier {
                                name: NamePartQualifier::Patronymic.into(),
                                value: None
                            }
                        ]
                    }]
                }]
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let data = TestData::new();

        let json = r#"{              
            "nameForms" : [ {            
            }],

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

        let name: Name = serde_json::from_str(json).unwrap();

        assert_eq!(
            name,
            Name {
                conclusion: data.conclusion_data,
                name_type: None,
                date: None, // TODO: Add in once we get the date type working
                name_forms: vec![NameForm {
                    lang: None,
                    full_text: None,
                    parts: vec![]
                }]
            }
        )
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let name = Name {
            conclusion: data.conclusion_data,
            name_type: Some(NameType::BirthName),
            date: None, // TODO: Add in once we get the date type working
            name_forms: vec![NameForm {
                lang: Some("en".to_string()),
                full_text: Some("full text of the name form".to_string()),
                parts: vec![NamePart {
                    part_type: Some(NamePartType::Surname),
                    value: "value of the name part".to_string(),
                    qualifiers: vec![
                        Qualifier {
                            name: NamePartQualifier::Family.into(),
                            value: None,
                        },
                        Qualifier {
                            name: NamePartQualifier::Patronymic.into(),
                            value: None,
                        },
                    ],
                }],
            }],
        };

        let json = serde_json::to_string(&name).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"type":"http://gedcomx.org/BirthName","nameForms":[{"lang":"en","fullText":"full text of the name form","parts":[{"type":"http://gedcomx.org/Surname","value":"value of the name part","qualifiers":[{"name":"http://gedcomx.org/Family"},{"name":"http://gedcomx.org/Patronymic"}]}]}]}"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let data = TestData::new();

        let name = Name {
            conclusion: data.conclusion_data,
            name_type: None,
            date: None,
            name_forms: vec![NameForm {
                lang: None,
                full_text: None,
                parts: vec![],
            }],
        };

        let json = serde_json::to_string(&name).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"nameForms":[{}]}"#
        )
    }
}

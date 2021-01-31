use super::EnumAsString;
use crate::{
    components::{Conclusion, ConclusionData, Date, Lang, Uri},
    Qualifier,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
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
    pub fn new(
        conclusion: ConclusionData,
        name_type: Option<NameType>,
        name_forms: Vec<NameForm>,
        date: Option<Date>,
    ) -> Self {
        Self {
            conclusion,
            name_type,
            name_forms,
            date,
        }
    }

    pub fn part_for_type(&self, name_type: &NamePartType) -> Option<&str> {
        self.name_forms.get(0)?.parts.iter().find_map(|n| {
            if n.part_type == Some(name_type.clone()) {
                Some(n.value.as_ref())
            } else {
                None
            }
        })
    }

    pub fn builder() -> NameBuilder {
        NameBuilder::new()
    }
}

impl Conclusion for Name {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("Name")
    }
}

pub struct NameBuilder(Name);

impl NameBuilder {
    pub(crate) fn new() -> Self {
        Self(Name::default())
    }

    conclusion_builder_functions!(Name);

    pub fn name_type(&mut self, name_type: NameType) -> &mut Self {
        self.0.name_type = Some(name_type);
        self
    }

    pub fn name_form(&mut self, name_form: NameForm) -> &mut Self {
        self.0.name_forms.push(name_form);
        self
    }

    pub fn name_forms(&mut self, name_forms: Vec<NameForm>) -> &mut Self {
        self.0.name_forms = name_forms;
        self
    }

    pub fn build(&self) -> Name {
        Name::new(
            self.0.conclusion.clone(),
            self.0.name_type.clone(),
            self.0.name_forms.clone(),
            self.0.date.clone(),
        )
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self {
            name_forms: vec![NameForm {
                full_text: Some(s.to_string()),
                ..NameForm::default()
            }],
            ..Self::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum NameType {
    BirthName,
    MarriedName,
    AlsoKnownAs,
    Nickname,
    AdoptiveName,
    FormalName,
    ReligiousName,
    Custom(Uri),
}

impl From<EnumAsString> for NameType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/BirthName" => Self::BirthName,
            "http://gedcomx.org/MarriedName" => Self::MarriedName,
            "http://gedcomx.org/AlsoKnownAs" => Self::AlsoKnownAs,
            "http://gedcomx.org/Nickname" => Self::Nickname,
            "http://gedcomx.org/AdoptiveName" => Self::AdoptiveName,
            "http://gedcomx.org/FormalName" => Self::FormalName,
            "http://gedcomx.org/ReligiousName" => Self::ReligiousName,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for NameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::BirthName => write!(f, "http://gedcomx.org/BirthName"),
            Self::MarriedName => write!(f, "http://gedcomx.org/MarriedName"),
            Self::AlsoKnownAs => write!(f, "http://gedcomx.org/AlsoKnownAs"),
            Self::Nickname => write!(f, "http://gedcomx.org/Nickname"),
            Self::AdoptiveName => write!(f, "http://gedcomx.org/AdoptiveName"),
            Self::FormalName => write!(f, "http://gedcomx.org/FormalName"),
            Self::ReligiousName => write!(f, "http://gedcomx.org/ReligiousName"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
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

impl NameForm {
    pub fn new(lang: Option<Lang>, full_text: Option<String>, parts: Vec<NamePart>) -> Self {
        Self {
            lang,
            full_text,
            parts,
        }
    }

    pub fn builder() -> NameFormBuilder {
        NameFormBuilder::new()
    }
}

pub struct NameFormBuilder(NameForm);

impl NameFormBuilder {
    pub(crate) fn new() -> Self {
        Self(NameForm::default())
    }

    pub fn full_text<I: Into<String>>(&mut self, full_text: I) -> &mut Self {
        self.0.full_text = Some(full_text.into());
        self
    }

    pub fn parts(&mut self, parts: Vec<NamePart>) -> &mut Self {
        self.0.parts = parts;
        self
    }

    pub fn part(&mut self, part: NamePart) -> &mut Self {
        self.0.parts.push(part);
        self
    }

    pub fn lang<I: Into<Lang>>(&mut self, lang: I) -> &mut Self {
        self.0.lang = Some(lang.into());
        self
    }

    pub fn build(&self) -> NameForm {
        NameForm::new(
            self.0.lang.clone(),
            self.0.full_text.clone(),
            self.0.parts.clone(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct NamePart {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub part_type: Option<NamePartType>,

    pub value: String,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub qualifiers: Vec<Qualifier>,
}

impl NamePart {
    pub fn new(part_type: Option<NamePartType>, value: String, qualifiers: Vec<Qualifier>) -> Self {
        Self {
            part_type,
            value,
            qualifiers,
        }
    }

    pub fn builder<I: Into<String>>(value: I) -> NamePartBuilder {
        NamePartBuilder::new(value)
    }
}

pub struct NamePartBuilder(NamePart);

impl NamePartBuilder {
    pub(crate) fn new<I: Into<String>>(value: I) -> Self {
        Self(NamePart {
            value: value.into(),
            ..NamePart::default()
        })
    }

    pub fn part_type(&mut self, part_type: NamePartType) -> &mut Self {
        self.0.part_type = Some(part_type);
        self
    }

    pub fn qualifier(&mut self, qualifier: Qualifier) -> &mut Self {
        self.0.qualifiers.push(qualifier);
        self
    }

    pub fn build(&self) -> NamePart {
        NamePart::new(
            self.0.part_type.clone(),
            self.0.value.clone(),
            self.0.qualifiers.clone(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum NamePartType {
    Prefix,
    Suffix,
    Given,
    Surname,
    Custom(Uri),
}

impl From<EnumAsString> for NamePartType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Prefix" => Self::Prefix,
            "http://gedcomx.org/Suffix" => Self::Suffix,
            "http://gedcomx.org/Given" => Self::Given,
            "http://gedcomx.org/Surname" => Self::Surname,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for NamePartType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Prefix => write!(f, "http://gedcomx.org/Prefix"),
            Self::Suffix => write!(f, "http://gedcomx.org/Suffix"),
            Self::Given => write!(f, "http://gedcomx.org/Given"),
            Self::Surname => write!(f, "http://gedcomx.org/Surname"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
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
    Custom(Uri),
}

impl From<EnumAsString> for NamePartQualifier {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Title" => Self::Title,
            "http://gedcomx.org/Primary" => Self::Primary,
            "http://gedcomx.org/Secondary" => Self::Secondary,
            "http://gedcomx.org/Middle" => Self::Middle,
            "http://gedcomx.org/Familiar" => Self::Familiar,
            "http://gedcomx.org/Religious" => Self::Religious,
            "http://gedcomx.org/Family" => Self::Family,
            "http://gedcomx.org/Maiden" => Self::Maiden,
            "http://gedcomx.org/Patronymic" => Self::Patronymic,
            "http://gedcomx.org/Matronymic" => Self::Matronymic,
            "http://gedcomx.org/Geographic" => Self::Geographic,
            "http://gedcomx.org/Occupational" => Self::Occupational,
            "http://gedcomx.org/Characteristic" => Self::Characteristic,
            "http://gedcomx.org/Postnom" => Self::Postnom,
            "http://gedcomx.org/Particle" => Self::Particle,
            "http://gedcomx.org/RootName" => Self::RootName,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for NamePartQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Title => write!(f, "http://gedcomx.org/Title"),
            Self::Primary => write!(f, "http://gedcomx.org/Primary"),
            Self::Secondary => write!(f, "http://gedcomx.org/Secondary"),
            Self::Middle => write!(f, "http://gedcomx.org/Middle"),
            Self::Familiar => write!(f, "http://gedcomx.org/Familiar"),
            Self::Religious => write!(f, "http://gedcomx.org/Religious"),
            Self::Family => write!(f, "http://gedcomx.org/Family"),
            Self::Maiden => write!(f, "http://gedcomx.org/Maiden"),
            Self::Patronymic => write!(f, "http://gedcomx.org/Patronymic"),
            Self::Matronymic => write!(f, "http://gedcomx.org/Matronymic"),
            Self::Geographic => write!(f, "http://gedcomx.org/Geographic"),
            Self::Occupational => write!(f, "http://gedcomx.org/Occupational"),
            Self::Characteristic => write!(f, "http://gedcomx.org/Characteristic"),
            Self::Postnom => write!(f, "http://gedcomx.org/Postnom"),
            Self::Particle => write!(f, "http://gedcomx.org/Particle"),
            Self::RootName => write!(f, "http://gedcomx.org/RootName"),
            Self::Custom(c) => write!(f, "{}", c),
        }
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

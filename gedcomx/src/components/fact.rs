use crate::{
    components::{Conclusion, ConclusionData, Date, PlaceReference, Uri},
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum FactType {
    // Person fact types.
    #[serde(rename = "http://gedcomx.org/Adoption")]
    Adoption,

    #[serde(rename = "http://gedcomx.org/AdultChristening")]
    AdultChristening,

    #[serde(rename = "http://gedcomx.org/Amnesty")]
    Amnesty,

    #[serde(rename = "http://gedcomx.org/AncestralHall")]
    AncestralHall,

    #[serde(rename = "http://gedcomx.org/AncestralPoem")]
    AncestralPoem,

    #[serde(rename = "http://gedcomx.org/Apprenticeship")]
    Apprenticeship,

    #[serde(rename = "http://gedcomx.org/Arrest")]
    Arrest,

    #[serde(rename = "http://gedcomx.org/Award")]
    Award,

    #[serde(rename = "http://gedcomx.org/Baptism")]
    Baptism,

    #[serde(rename = "http://gedcomx.org/BarMitzvah")]
    BarMitzvah,

    #[serde(rename = "http://gedcomx.org/BatMitzvah")]
    BatMitzvah,

    #[serde(rename = "http://gedcomx.org/Birth")]
    Birth,

    #[serde(rename = "http://gedcomx.org/BirthNotice")]
    BirthNotice,

    #[serde(rename = "http://gedcomx.org/Blessing")]
    Blessing,

    #[serde(rename = "http://gedcomx.org/Branch")]
    Branch,

    #[serde(rename = "http://gedcomx.org/Burial")]
    Burial,

    #[serde(rename = "http://gedcomx.org/Caste")]
    Caste,

    #[serde(rename = "http://gedcomx.org/Census")]
    Census,

    #[serde(rename = "http://gedcomx.org/Christening")]
    Christening,

    #[serde(rename = "http://gedcomx.org/Circumcision")]
    Circumcision,

    #[serde(rename = "http://gedcomx.org/Clan")]
    Clan,

    #[serde(rename = "http://gedcomx.org/Confirmation")]
    Confirmation,

    #[serde(rename = "http://gedcomx.org/Court")]
    Court,

    #[serde(rename = "http://gedcomx.org/Cremation")]
    Cremation,

    #[serde(rename = "http://gedcomx.org/Death")]
    Death,

    #[serde(rename = "http://gedcomx.org/Education")]
    Education,

    #[serde(rename = "http://gedcomx.org/EducationEnrollment")]
    EducationEnrollment,

    #[serde(rename = "http://gedcomx.org/Emigration")]
    Emigration,

    #[serde(rename = "http://gedcomx.org/Enslavement")]
    Enslavement,

    #[serde(rename = "http://gedcomx.org/Ethnicity")]
    Ethnicity,

    #[serde(rename = "http://gedcomx.org/Excommunication")]
    Excommunication,

    #[serde(rename = "http://gedcomx.org/FirstCommunion")]
    FirstCommunion,

    #[serde(rename = "http://gedcomx.org/Funeral")]
    Funeral,

    #[serde(rename = "http://gedcomx.org/GenderChange")]
    GenderChange,

    #[serde(rename = "http://gedcomx.org/GenerationNumber")]
    GenerationNumber,

    #[serde(rename = "http://gedcomx.org/Graduation")]
    Graduation,

    #[serde(rename = "http://gedcomx.org/Heimat")]
    Heimat,

    #[serde(rename = "http://gedcomx.org/Immigration")]
    Immigration,

    #[serde(rename = "http://gedcomx.org/Imprisonment")]
    Imprisonment,

    #[serde(rename = "http://gedcomx.org/Inquest")]
    Inquest,

    #[serde(rename = "http://gedcomx.org/LandTransaction")]
    LandTransaction,

    #[serde(rename = "http://gedcomx.org/Language")]
    Language,

    #[serde(rename = "http://gedcomx.org/Living")]
    Living,

    #[serde(rename = "http://gedcomx.org/MaritalStatus")]
    MaritalStatus,

    #[serde(rename = "http://gedcomx.org/Medical")]
    Medical,

    #[serde(rename = "http://gedcomx.org/MilitaryAward")]
    MilitaryAward,

    #[serde(rename = "http://gedcomx.org/MilitaryDischarge")]
    MilitaryDischarge,

    #[serde(rename = "http://gedcomx.org/MilitaryDraftRegistration")]
    MilitaryDraftRegistration,

    #[serde(rename = "http://gedcomx.org/MilitaryInduction")]
    MilitaryInduction,

    #[serde(rename = "http://gedcomx.org/MilitaryService")]
    MilitaryService,

    #[serde(rename = "http://gedcomx.org/Mission")]
    Mission,

    #[serde(rename = "http://gedcomx.org/MoveFrom")]
    MoveFrom,

    #[serde(rename = "http://gedcomx.org/MoveTo")]
    MoveTo,

    #[serde(rename = "http://gedcomx.org/MultipleBirth")]
    MultipleBirth,

    #[serde(rename = "http://gedcomx.org/NationalId")]
    NationalId,

    #[serde(rename = "http://gedcomx.org/Nationality")]
    Nationality,

    #[serde(rename = "http://gedcomx.org/Naturalization")]
    Naturalization,

    #[serde(rename = "http://gedcomx.org/NumberOfChildren")]
    NumberOfChildren, // Also a couple fact type.

    #[serde(rename = "http://gedcomx.org/NumberOfMarriages")]
    NumberOfMarriages,

    #[serde(rename = "http://gedcomx.org/Obituary")]
    Obituary,

    #[serde(rename = "http://gedcomx.org/OfficialPosition")]
    OfficialPosition,

    #[serde(rename = "http://gedcomx.org/Occupation")]
    Occupation,

    #[serde(rename = "http://gedcomx.org/Ordination")]
    Ordination,

    #[serde(rename = "http://gedcomx.org/Pardon")]
    Pardon,

    #[serde(rename = "http://gedcomx.org/PhysicalDescription")]
    PhysicalDescription,

    #[serde(rename = "http://gedcomx.org/Probate")]
    Probate,

    #[serde(rename = "http://gedcomx.org/Property")]
    Property,

    #[serde(rename = "http://gedcomx.org/Race")]
    Race,

    #[serde(rename = "http://gedcomx.org/Religion")]
    Religion,

    #[serde(rename = "http://gedcomx.org/Residence")]
    Residence,

    #[serde(rename = "http://gedcomx.org/Retirement")]
    Retirement,

    #[serde(rename = "http://gedcomx.org/Stillbirth")]
    Stillbirth,

    #[serde(rename = "http://gedcomx.org/TaxAssessment")]
    TaxAssessment,

    #[serde(rename = "http://gedcomx.org/Tribe")]
    Tribe,

    #[serde(rename = "http://gedcomx.org/Will")]
    Will,

    #[serde(rename = "http://gedcomx.org/Visit")]
    Visit,

    #[serde(rename = "http://gedcomx.org/Yahrzeit")]
    Yahrzeit,

    // Couple fact types.
    #[serde(rename = "http://gedcomx.org/Annulment")]
    Annulment,

    #[serde(rename = "http://gedcomx.org/CommonLawMarriage")]
    CommonLawMarriage,

    #[serde(rename = "http://gedcomx.org/CivilUnion")]
    CivilUnion,

    #[serde(rename = "http://gedcomx.org/Divorce")]
    Divorce,

    #[serde(rename = "http://gedcomx.org/DivorceFiling")]
    DivorceFiling,

    #[serde(rename = "http://gedcomx.org/DomesticPartnership")]
    DomesticPartnership,

    #[serde(rename = "http://gedcomx.org/Engagement")]
    Engagement,

    #[serde(rename = "http://gedcomx.org/Marriage")]
    Marriage,

    #[serde(rename = "http://gedcomx.org/MarriageBanns")]
    MarriageBanns,

    #[serde(rename = "http://gedcomx.org/MarriageContract")]
    MarriageContract,

    #[serde(rename = "http://gedcomx.org/MarriageLicense")]
    MarriageLicense,

    #[serde(rename = "http://gedcomx.org/MarriageNotice")]
    MarriageNotice,

    #[serde(rename = "http://gedcomx.org/Separation")]
    Separation,

    // Parent-child fact types.
    #[serde(rename = "http://gedcomx.org/AdoptiveParent")]
    AdoptiveParent,

    #[serde(rename = "http://gedcomx.org/BiologicalParent")]
    BiologicalParent,

    #[serde(rename = "http://gedcomx.org/ChildOrder")]
    ChildOrder,

    #[serde(rename = "http://gedcomx.org/EnteringHeir")]
    EnteringHeir,

    #[serde(rename = "http://gedcomx.org/ExitingHeir")]
    ExitingHeir,

    #[serde(rename = "http://gedcomx.org/FosterParent")]
    FosterParent,

    #[serde(rename = "http://gedcomx.org/GuardianParent")]
    GuardianParent,

    #[serde(rename = "http://gedcomx.org/StepParent")]
    StepParent,

    #[serde(rename = "http://gedcomx.org/SociologicalParent")]
    SociologicalParent,

    #[serde(rename = "http://gedcomx.org/SurrogateParent")]
    SurrogateParent,
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

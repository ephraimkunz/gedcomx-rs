use crate::{
    components::{Conclusion, ConclusionData, Date, EnumAsString, Id, PlaceReference, Uri},
    Qualifier,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
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

impl Fact {
    pub fn new(
        fact_type: FactType,
        conclusion: ConclusionData,
        date: Option<Date>,
        place: Option<PlaceReference>,
        value: Option<String>,
        qualifiers: Vec<Qualifier>,
    ) -> Self {
        Self {
            fact_type,
            conclusion,
            date,
            place,
            value,
            qualifiers,
        }
    }

    pub fn builder(fact_type: FactType) -> FactBuilder {
        FactBuilder::new(fact_type)
    }
}

pub struct FactBuilder(Fact);

impl FactBuilder {
    pub(crate) fn new(fact_type: FactType) -> Self {
        Self(Fact {
            fact_type,
            ..Fact::default()
        })
    }

    pub fn date(&mut self, date: Date) -> &mut Self {
        self.0.date = Some(date);
        self
    }

    pub fn place(&mut self, place: PlaceReference) -> &mut Self {
        self.0.place = Some(place);
        self
    }

    pub fn value<I: Into<String>>(&mut self, value: I) -> &mut Self {
        self.0.value = Some(value.into());
        self
    }

    pub fn id<I: Into<Id>>(&mut self, id: I) -> &mut Self {
        self.0.conclusion.id = Some(id.into());
        self
    }

    pub fn build(&self) -> Fact {
        Fact::new(
            self.0.fact_type.clone(),
            self.0.conclusion.clone(),
            self.0.date.clone(),
            self.0.place.clone(),
            self.0.value.clone(),
            self.0.qualifiers.clone(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum FactType {
    // Person fact types.
    Adoption,
    AdultChristening,
    Amnesty,
    AncestralHall,
    AncestralPoem,
    Apprenticeship,
    Arrest,
    Award,
    Baptism,
    BarMitzvah,
    BatMitzvah,
    Birth,
    BirthNotice,
    Blessing,
    Branch,
    Burial,
    Caste,
    Census,
    Christening,
    Circumcision,
    Clan,
    Confirmation,
    Court,
    Cremation,
    Death,
    Education,
    EducationEnrollment,
    Emigration,
    Enslavement,
    Ethnicity,
    Excommunication,
    FirstCommunion,
    Funeral,
    GenderChange,
    GenerationNumber,
    Graduation,
    Heimat,
    Immigration,
    Imprisonment,
    Inquest,
    LandTransaction,
    Language,
    Living,
    MaritalStatus,
    Medical,
    MilitaryAward,
    MilitaryDischarge,
    MilitaryDraftRegistration,
    MilitaryInduction,
    MilitaryService,
    Mission,
    MoveFrom,
    MoveTo,
    MultipleBirth,
    NationalId,
    Nationality,
    Naturalization,
    NumberOfChildren, // Also a couple fact type.
    NumberOfMarriages,
    Obituary,
    OfficialPosition,
    Occupation,
    Ordination,
    Pardon,
    PhysicalDescription,
    Probate,
    Property,
    Race,
    Religion,
    Residence,
    Retirement,
    Stillbirth,
    TaxAssessment,
    Tribe,
    Will,
    Visit,
    Yahrzeit,

    // Couple fact types.
    Annulment,
    CommonLawMarriage,
    CivilUnion,
    Divorce,
    DivorceFiling,
    DomesticPartnership,
    Engagement,
    Marriage,
    MarriageBanns,
    MarriageContract,
    MarriageLicense,
    MarriageNotice,
    Separation,

    // Parent-child fact types.
    AdoptiveParent,
    BiologicalParent,
    ChildOrder,
    EnteringHeir,
    ExitingHeir,
    FosterParent,
    GuardianParent,
    StepParent,
    SociologicalParent,
    SurrogateParent,

    // Catch all
    Custom(Uri),
}

impl fmt::Display for FactType {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Adoption => write!(f, "http://gedcomx.org/Adoption"),
            Self::AdultChristening => write!(f, "http://gedcomx.org/AdultChristening"),
            Self::Amnesty => write!(f, "http://gedcomx.org/Amnesty"),
            Self::AncestralHall => write!(f, "http://gedcomx.org/AncestralHall"),
            Self::AncestralPoem => write!(f, "http://gedcomx.org/AncestralPoem"),
            Self::Apprenticeship => write!(f, "http://gedcomx.org/Apprenticeship"),
            Self::Arrest => write!(f, "http://gedcomx.org/Arrest"),
            Self::Award => write!(f, "http://gedcomx.org/Award"),
            Self::Baptism => write!(f, "http://gedcomx.org/Baptism"),
            Self::BarMitzvah => write!(f, "http://gedcomx.org/BarMitzvah"),
            Self::BatMitzvah => write!(f, "http://gedcomx.org/BatMitzvah"),
            Self::Birth => write!(f, "http://gedcomx.org/Birth"),
            Self::BirthNotice => write!(f, "http://gedcomx.org/BirthNotice"),
            Self::Blessing => write!(f, "http://gedcomx.org/Blessing"),
            Self::Branch => write!(f, "http://gedcomx.org/Branch"),
            Self::Burial => write!(f, "http://gedcomx.org/Burial"),
            Self::Caste => write!(f, "http://gedcomx.org/Caste"),
            Self::Census => write!(f, "http://gedcomx.org/Census"),
            Self::Christening => write!(f, "http://gedcomx.org/Christening"),
            Self::Circumcision => write!(f, "http://gedcomx.org/Circumcision"),
            Self::Clan => write!(f, "http://gedcomx.org/Clan"),
            Self::Confirmation => write!(f, "http://gedcomx.org/Confirmation"),
            Self::Court => write!(f, "http://gedcomx.org/Court"),
            Self::Cremation => write!(f, "http://gedcomx.org/Cremation"),
            Self::Death => write!(f, "http://gedcomx.org/Death"),
            Self::Education => write!(f, "http://gedcomx.org/Education"),
            Self::EducationEnrollment => write!(f, "http://gedcomx.org/EducationEnrollment"),
            Self::Emigration => write!(f, "http://gedcomx.org/Emigration"),
            Self::Enslavement => write!(f, "http://gedcomx.org/Enslavement"),
            Self::Ethnicity => write!(f, "http://gedcomx.org/Ethnicity"),
            Self::Excommunication => write!(f, "http://gedcomx.org/Excommunication"),
            Self::FirstCommunion => write!(f, "http://gedcomx.org/FirstCommunion"),
            Self::Funeral => write!(f, "http://gedcomx.org/Funeral"),
            Self::GenderChange => write!(f, "http://gedcomx.org/GenderChange"),
            Self::GenerationNumber => write!(f, "http://gedcomx.org/GenerationNumber"),
            Self::Graduation => write!(f, "http://gedcomx.org/Graduation"),
            Self::Heimat => write!(f, "http://gedcomx.org/Heimat"),
            Self::Immigration => write!(f, "http://gedcomx.org/Immigration"),
            Self::Imprisonment => write!(f, "http://gedcomx.org/Imprisonment"),
            Self::Inquest => write!(f, "http://gedcomx.org/Inquest"),
            Self::LandTransaction => write!(f, "http://gedcomx.org/LandTransaction"),
            Self::Language => write!(f, "http://gedcomx.org/Language"),
            Self::Living => write!(f, "http://gedcomx.org/Living"),
            Self::MaritalStatus => write!(f, "http://gedcomx.org/MaritalStatus"),
            Self::Medical => write!(f, "http://gedcomx.org/Medical"),
            Self::MilitaryAward => write!(f, "http://gedcomx.org/MilitaryAward"),
            Self::MilitaryDischarge => write!(f, "http://gedcomx.org/MilitaryDischarge"),
            Self::MilitaryDraftRegistration => {
                write!(f, "http://gedcomx.org/MilitaryDraftRegistration")
            }
            Self::MilitaryInduction => write!(f, "http://gedcomx.org/MilitaryInduction"),
            Self::MilitaryService => write!(f, "http://gedcomx.org/MilitaryService"),
            Self::Mission => write!(f, "http://gedcomx.org/Mission"),
            Self::MoveFrom => write!(f, "http://gedcomx.org/MoveFrom"),
            Self::MoveTo => write!(f, "http://gedcomx.org/MoveTo"),
            Self::MultipleBirth => write!(f, "http://gedcomx.org/MultipleBirth"),
            Self::NationalId => write!(f, "http://gedcomx.org/NationalId"),
            Self::Nationality => write!(f, "http://gedcomx.org/Nationality"),
            Self::Naturalization => write!(f, "http://gedcomx.org/Naturalization"),
            Self::NumberOfChildren => write!(f, "http://gedcomx.org/NumberOfChildren"),
            Self::NumberOfMarriages => write!(f, "http://gedcomx.org/NumberOfMarriages"),
            Self::Obituary => write!(f, "http://gedcomx.org/Obituary"),
            Self::OfficialPosition => write!(f, "http://gedcomx.org/OfficialPosition"),
            Self::Occupation => write!(f, "http://gedcomx.org/Occupation"),
            Self::Ordination => write!(f, "http://gedcomx.org/Ordination"),
            Self::Pardon => write!(f, "http://gedcomx.org/Pardon"),
            Self::PhysicalDescription => write!(f, "http://gedcomx.org/PhysicalDescription"),
            Self::Probate => write!(f, "http://gedcomx.org/Probate"),
            Self::Property => write!(f, "http://gedcomx.org/Property"),
            Self::Race => write!(f, "http://gedcomx.org/Race"),
            Self::Religion => write!(f, "http://gedcomx.org/Religion"),
            Self::Residence => write!(f, "http://gedcomx.org/Residence"),
            Self::Retirement => write!(f, "http://gedcomx.org/Retirement"),
            Self::Stillbirth => write!(f, "http://gedcomx.org/Stillbirth"),
            Self::TaxAssessment => write!(f, "http://gedcomx.org/TaxAssessment"),
            Self::Tribe => write!(f, "http://gedcomx.org/Tribe"),
            Self::Will => write!(f, "http://gedcomx.org/Will"),
            Self::Visit => write!(f, "http://gedcomx.org/Visit"),
            Self::Yahrzeit => write!(f, "http://gedcomx.org/Yahrzeit"),
            Self::Annulment => write!(f, "http://gedcomx.org/Annulment"),
            Self::CommonLawMarriage => write!(f, "http://gedcomx.org/CommonLawMarriage"),
            Self::CivilUnion => write!(f, "http://gedcomx.org/CivilUnion"),
            Self::Divorce => write!(f, "http://gedcomx.org/Divorce"),
            Self::DivorceFiling => write!(f, "http://gedcomx.org/DivorceFiling"),
            Self::DomesticPartnership => write!(f, "http://gedcomx.org/DomesticPartnership"),
            Self::Engagement => write!(f, "http://gedcomx.org/Engagement"),
            Self::Marriage => write!(f, "http://gedcomx.org/Marriage"),
            Self::MarriageBanns => write!(f, "http://gedcomx.org/MarriageBanns"),
            Self::MarriageContract => write!(f, "http://gedcomx.org/MarriageContract"),
            Self::MarriageLicense => write!(f, "http://gedcomx.org/MarriageLicense"),
            Self::MarriageNotice => write!(f, "http://gedcomx.org/MarriageNotice"),
            Self::Separation => write!(f, "http://gedcomx.org/Separation"),
            Self::AdoptiveParent => write!(f, "http://gedcomx.org/AdoptiveParent"),
            Self::BiologicalParent => write!(f, "http://gedcomx.org/BiologicalParent"),
            Self::ChildOrder => write!(f, "http://gedcomx.org/ChildOrder"),
            Self::EnteringHeir => write!(f, "http://gedcomx.org/EnteringHeir"),
            Self::ExitingHeir => write!(f, "http://gedcomx.org/ExitingHeir"),
            Self::FosterParent => write!(f, "http://gedcomx.org/FosterParent"),
            Self::GuardianParent => write!(f, "http://gedcomx.org/GuardianParent"),
            Self::StepParent => write!(f, "http://gedcomx.org/StepParent"),
            Self::SociologicalParent => write!(f, "http://gedcomx.org/SociologicalParent"),
            Self::SurrogateParent => write!(f, "http://gedcomx.org/SurrogateParent"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

impl From<EnumAsString> for FactType {
    #[allow(clippy::too_many_lines)]
    fn from(f: EnumAsString) -> Self {
        // If you need to generate this mapping in the future, the easiest way is to copy and paste the tables in
        // https://github.com/FamilySearch/gedcomx/blob/master/specifications/fact-types-specification.md.
        // Then use VSCode's find and replace with regex feature with a find regex: (http://gedcomx.org/([a-zA-Z]+)).*
        // and a replace regex: "$1" => Self::$2,
        match f.0.as_ref() {
            "http://gedcomx.org/Adoption" => Self::Adoption,
            "http://gedcomx.org/AdultChristening" => Self::AdultChristening,
            "http://gedcomx.org/Amnesty" => Self::Amnesty,
            "http://gedcomx.org/AncestralHall" => Self::AncestralHall,
            "http://gedcomx.org/AncestralPoem" => Self::AncestralPoem,
            "http://gedcomx.org/Apprenticeship" => Self::Apprenticeship,
            "http://gedcomx.org/Arrest" => Self::Arrest,
            "http://gedcomx.org/Award" => Self::Award,
            "http://gedcomx.org/Baptism" => Self::Baptism,
            "http://gedcomx.org/BarMitzvah" => Self::BarMitzvah,
            "http://gedcomx.org/BatMitzvah" => Self::BatMitzvah,
            "http://gedcomx.org/Birth" => Self::Birth,
            "http://gedcomx.org/BirthNotice" => Self::BirthNotice,
            "http://gedcomx.org/Blessing" => Self::Blessing,
            "http://gedcomx.org/Branch" => Self::Branch,
            "http://gedcomx.org/Burial" => Self::Burial,
            "http://gedcomx.org/Caste" => Self::Caste,
            "http://gedcomx.org/Census" => Self::Census,
            "http://gedcomx.org/Christening" => Self::Christening,
            "http://gedcomx.org/Circumcision" => Self::Circumcision,
            "http://gedcomx.org/Clan" => Self::Clan,
            "http://gedcomx.org/Confirmation" => Self::Confirmation,
            "http://gedcomx.org/Court" => Self::Court,
            "http://gedcomx.org/Cremation" => Self::Cremation,
            "http://gedcomx.org/Death" => Self::Death,
            "http://gedcomx.org/Education" => Self::Education,
            "http://gedcomx.org/EducationEnrollment" => Self::EducationEnrollment,
            "http://gedcomx.org/Emigration" => Self::Emigration,
            "http://gedcomx.org/Enslavement" => Self::Enslavement,
            "http://gedcomx.org/Ethnicity" => Self::Ethnicity,
            "http://gedcomx.org/Excommunication" => Self::Excommunication,
            "http://gedcomx.org/FirstCommunion" => Self::FirstCommunion,
            "http://gedcomx.org/Funeral" => Self::Funeral,
            "http://gedcomx.org/GenderChange" => Self::GenderChange,
            "http://gedcomx.org/GenerationNumber" => Self::GenerationNumber,
            "http://gedcomx.org/Graduation" => Self::Graduation,
            "http://gedcomx.org/Heimat" => Self::Heimat,
            "http://gedcomx.org/Immigration" => Self::Immigration,
            "http://gedcomx.org/Imprisonment" => Self::Imprisonment,
            "http://gedcomx.org/Inquest" => Self::Inquest,
            "http://gedcomx.org/LandTransaction" => Self::LandTransaction,
            "http://gedcomx.org/Language" => Self::Language,
            "http://gedcomx.org/Living" => Self::Living,
            "http://gedcomx.org/MaritalStatus" => Self::MaritalStatus,
            "http://gedcomx.org/Medical" => Self::Medical,
            "http://gedcomx.org/MilitaryAward" => Self::MilitaryAward,
            "http://gedcomx.org/MilitaryDischarge" => Self::MilitaryDischarge,
            "http://gedcomx.org/MilitaryDraftRegistration" => Self::MilitaryDraftRegistration,
            "http://gedcomx.org/MilitaryInduction" => Self::MilitaryInduction,
            "http://gedcomx.org/MilitaryService" => Self::MilitaryService,
            "http://gedcomx.org/Mission" => Self::Mission,
            "http://gedcomx.org/MoveFrom" => Self::MoveFrom,
            "http://gedcomx.org/MoveTo" => Self::MoveTo,
            "http://gedcomx.org/MultipleBirth" => Self::MultipleBirth,
            "http://gedcomx.org/NationalId" => Self::NationalId,
            "http://gedcomx.org/Nationality" => Self::Nationality,
            "http://gedcomx.org/Naturalization" => Self::Naturalization,
            "http://gedcomx.org/NumberOfChildren" => Self::NumberOfChildren,
            "http://gedcomx.org/NumberOfMarriages" => Self::NumberOfMarriages,
            "http://gedcomx.org/Obituary" => Self::Obituary,
            "http://gedcomx.org/OfficialPosition" => Self::OfficialPosition,
            "http://gedcomx.org/Occupation" => Self::Occupation,
            "http://gedcomx.org/Ordination" => Self::Ordination,
            "http://gedcomx.org/Pardon" => Self::Pardon,
            "http://gedcomx.org/PhysicalDescription" => Self::PhysicalDescription,
            "http://gedcomx.org/Probate" => Self::Probate,
            "http://gedcomx.org/Property" => Self::Property,
            "http://gedcomx.org/Race" => Self::Race,
            "http://gedcomx.org/Religion" => Self::Religion,
            "http://gedcomx.org/Residence" => Self::Residence,
            "http://gedcomx.org/Retirement" => Self::Retirement,
            "http://gedcomx.org/Stillbirth" => Self::Stillbirth,
            "http://gedcomx.org/TaxAssessment" => Self::TaxAssessment,
            "http://gedcomx.org/Tribe" => Self::Tribe,
            "http://gedcomx.org/Will" => Self::Will,
            "http://gedcomx.org/Visit" => Self::Visit,
            "http://gedcomx.org/Yahrzeit" => Self::Yahrzeit,
            "http://gedcomx.org/Annulment" => Self::Annulment,
            "http://gedcomx.org/CommonLawMarriage" => Self::CommonLawMarriage,
            "http://gedcomx.org/CivilUnion" => Self::CivilUnion,
            "http://gedcomx.org/Divorce" => Self::Divorce,
            "http://gedcomx.org/DivorceFiling" => Self::DivorceFiling,
            "http://gedcomx.org/DomesticPartnership" => Self::DomesticPartnership,
            "http://gedcomx.org/Engagement" => Self::Engagement,
            "http://gedcomx.org/Marriage" => Self::Marriage,
            "http://gedcomx.org/MarriageBanns" => Self::MarriageBanns,
            "http://gedcomx.org/MarriageContract" => Self::MarriageContract,
            "http://gedcomx.org/MarriageLicense" => Self::MarriageLicense,
            "http://gedcomx.org/MarriageNotice" => Self::MarriageNotice,
            "http://gedcomx.org/Separation" => Self::Separation,
            "http://gedcomx.org/AdoptiveParent" => Self::AdoptiveParent,
            "http://gedcomx.org/BiologicalParent" => Self::BiologicalParent,
            "http://gedcomx.org/ChildOrder" => Self::ChildOrder,
            "http://gedcomx.org/EnteringHeir" => Self::EnteringHeir,
            "http://gedcomx.org/ExitingHeir" => Self::ExitingHeir,
            "http://gedcomx.org/FosterParent" => Self::FosterParent,
            "http://gedcomx.org/GuardianParent" => Self::GuardianParent,
            "http://gedcomx.org/StepParent" => Self::StepParent,
            "http://gedcomx.org/SociologicalParent" => Self::SociologicalParent,
            "http://gedcomx.org/SurrogateParent" => Self::SurrogateParent,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl Default for FactType {
    fn default() -> Self {
        Self::Custom(Uri::from(String::default()))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum FactQualifier {
    Age,
    Cause,
    Religion,
    Transport,
    NonConsensual,
    Custom(Uri),
}

impl From<EnumAsString> for FactQualifier {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Age" => Self::Age,
            "http://gedcomx.org/Cause" => Self::Cause,
            "http://gedcomx.org/Religion" => Self::Religion,
            "http://gedcomx.org/Transport" => Self::Transport,
            "http://gedcomx.org/NonConsensual" => Self::NonConsensual,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for FactQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Age => write!(f, "http://gedcomx.org/Age"),
            Self::Cause => write!(f, "http://gedcomx.org/Cause"),
            Self::Religion => write!(f, "http://gedcomx.org/Religion"),
            Self::Transport => write!(f, "http://gedcomx.org/Transport"),
            Self::NonConsensual => write!(f, "http://gedcomx.org/NonConsensual"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::TestData;

    #[test]
    fn json_serialize_custom_fact_type() {
        let t = FactType::Custom("this is a custom fact".into());
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, r#""this is a custom fact""#)
    }

    #[test]
    fn json_deserialize_custom_fact_type() {
        let json = r#""this is a custom fact""#;
        let t: FactType = serde_json::from_str(json).unwrap();
        assert_eq!(t, FactType::Custom("this is a custom fact".into()))
    }

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

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

// I can't figure out how to get Serde to properly serialize the custom enum variant of FactType with annotations,
// so rather than write a Deserializer / Serializer implementation we'll just serialize to this newtype and then
// Serde will automatically convert it to FactType.
#[derive(Serialize, Deserialize)]
struct FactTypeSerde(String);

impl From<FactType> for FactTypeSerde {
    fn from(f: FactType) -> Self {
        Self(f.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "FactTypeSerde", into = "FactTypeSerde")]
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

    Custom(Uri),
}

impl fmt::Display for FactType {
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

impl From<FactTypeSerde> for FactType {
    fn from(f: FactTypeSerde) -> Self {
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

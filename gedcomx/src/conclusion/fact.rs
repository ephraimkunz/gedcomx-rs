use std::{fmt, str::FromStr};

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, Date, EnumAsString, GedcomxError, Id, Lang, Note, PlaceReference,
    Qualifier, ResourceReference, Result, SourceReference, Uri,
};

/// A data item that is presumed to be true about a specific subject, such as a
/// person or relationship.
///
/// # Events Versus Facts
///
/// GEDCOM X implementations need to be able to recognize the difference between
/// the concept of an "event" and the concept of a "fact" as defined by this
/// specification in order to correctly use the data types associated with these
/// concepts. This section is provided for the purpose of explicitly defining
/// and distinguishing the two concepts.
///
/// An "event" is an occurrence that happened at a specific time or period of
/// time, often at a specific place or set of places. Genealogically relevant
/// events are often described by referencing the persons that played a role in
/// that event. Hence events often refer to persons and might infer
/// relationships, but events are described independently of those persons and
/// relationships.
///
/// A "fact" is a data item that is presumed to be true about a specific
/// subject, such as a person or relationship. A time or place is often, but not
/// always, applicable to a fact. Facts do not exist outside the scope of the
/// subject to which they apply.
///
/// Events are often used to infer facts. A marriage event, for example, infers
/// the fact that two persons were married, and birth event infers the fact that
/// a person was born. Facts also sometimes infer events, but the existence of a
/// fact might not always justify a description of an event. For example, a
/// birth fact provided by a census record might not warrant a description of a
/// birth event, even though the existence of such an event is implied. On the
/// other hand, a birth record that provides information about biological
/// parents, adoptive parents, additional witnesses, etc. might justify a
/// description of the event in addition to descriptions of any facts provided
/// by the record.
///
/// Despite the occasional inference of facts from events and vice versa, this
/// specification dictates that the two concepts are described independently.
/// This version of the specification does not provide a direct association
/// between instances of the two data types, although an indirect association
/// can be found via the event role.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct Fact {
    /// An identifier for the conclusion data. The id is to be used as a "fragment identifier" as defined by [RFC 3986, Section 3.5](https://tools.ietf.org/html/rfc3986#section-3.5).
    #[yaserde(attribute)]
    pub id: Option<Id>,

    /// The locale identifier for the conclusion.
    #[yaserde(attribute, prefix = "xml")]
    pub lang: Option<Lang>,

    /// The list of references to the sources of related to this conclusion.
    /// Note that the sources referenced from conclusions are also considered
    /// to be sources of the entities that contain them. For example, a source
    /// associated with the `Name` of a `Person` is also source for the
    /// `Person`.
    #[yaserde(rename = "source", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    /// A reference to the analysis document explaining the analysis that went
    /// into this conclusion. If provided, MUST resolve to an instance of
    /// [Document](crate::Document) of type
    /// [Analysis](crate::DocumentType::Analysis).
    #[yaserde(prefix = "gx")]
    pub analysis: Option<ResourceReference>,

    /// A list of notes about this conclusion.
    #[yaserde(rename = "note", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub notes: Vec<Note>,

    /// The level of confidence the contributor has about the data.
    #[yaserde(attribute)]
    pub confidence: Option<ConfidenceLevel>,

    /// The attribution of this conclusion.
    /// If not provided, the attribution of the containing data set (e.g. file)
    /// of the conclusion is assumed.
    #[yaserde(prefix = "gx")]
    pub attribution: Option<Attribution>,

    /// The type of the fact.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub fact_type: FactType,

    /// The date of applicability of the fact.
    #[yaserde(prefix = "gx")]
    pub date: Option<Date>,

    /// A reference to the place applicable to this fact.
    #[yaserde(prefix = "gx")]
    pub place: Option<PlaceReference>,

    /// The value of the fact.
    #[yaserde(prefix = "gx")]
    pub value: Option<String>,

    /// Qualifiers to add additional details about the fact.
    ///
    /// If present, use of a
    /// [`FactQualifier`](crate::FactQualifier) is
    /// RECOMMENDED.
    #[yaserde(rename = "qualifier", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub qualifiers: Vec<Qualifier>,
}

impl Fact {
    pub fn new(
        id: Option<Id>,
        lang: Option<Lang>,
        sources: Vec<SourceReference>,
        analysis: Option<ResourceReference>,
        notes: Vec<Note>,
        confidence: Option<ConfidenceLevel>,
        attribution: Option<Attribution>,
        fact_type: FactType,
        date: Option<Date>,
        place: Option<PlaceReference>,
        value: Option<String>,
        qualifiers: Vec<Qualifier>,
    ) -> Self {
        Self {
            id,
            lang,
            sources,
            analysis,
            notes,
            confidence,
            attribution,
            fact_type,
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

impl Arbitrary for Fact {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut fact = Self::builder(FactType::arbitrary(g))
            .id(Id::arbitrary(g))
            .lang(Lang::arbitrary(g))
            .note(Note::arbitrary(g))
            .confidence(ConfidenceLevel::arbitrary(g))
            .attribution(Attribution::arbitrary(g))
            .date(Date::arbitrary(g))
            .place(PlaceReference::arbitrary(g))
            .value(crate::arbitrary_trimmed(g))
            .qualifier(Qualifier::arbitrary(g))
            .build();

        fact.analysis = Some(ResourceReference::arbitrary(g));
        fact.sources = vec![SourceReference::arbitrary(g)];

        fact
    }
}

pub struct FactBuilder(Fact);

impl FactBuilder {
    conclusion_builder_functions!(Fact);

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

    pub fn qualifier(&mut self, qualifier: Qualifier) -> &mut Self {
        self.0.qualifiers.push(qualifier);
        self
    }

    pub fn build(&self) -> Fact {
        Fact::new(
            self.0.id.clone(),
            self.0.lang.clone(),
            self.0.sources.clone(),
            self.0.analysis.clone(),
            self.0.notes.clone(),
            self.0.confidence.clone(),
            self.0.attribution.clone(),
            self.0.fact_type.clone(),
            self.0.date.clone(),
            self.0.place.clone(),
            self.0.value.clone(),
            self.0.qualifiers.clone(),
        )
    }
}

/// Standard fact types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum FactType {
    // Person fact types.
    /// In the context of a parent-child relationship, it describes a fact of
    /// the adoption of a child by a parent.
    Adoption,

    /// A fact of a person's christening or baptism as an adult.
    AdultChristening,
    Amnesty,

    /// A fact of a person's ancestral hall.
    ///
    /// An ancestral hall refers to a location where the early ancestors of the
    /// person originated. It may also refer to the name of an early ancestor.
    /// Family clans are often distinguished one from another by the ancestral
    /// hall. Clans that cannot prove direct relationships to other clans with
    /// the same surname can assume a direct relationship if they share the same
    /// ancestral hall.
    AncestralHall,

    /// A fact of a person's ancestral poem.
    ///
    /// An ancestral poem (or generation poem) is composed of the "generation characters" that are to be used when choosing names for the members of different
    ///  generations of an extended family. Ancestral poems are prominent in
    /// Asian countries, particularly China.
    AncestralPoem,
    Apprenticeship,
    Arrest,

    /// A fact of a person's award (medal, honor).
    Award,
    Baptism,
    BarMitzvah,
    BatMitzvah,
    Birth,

    /// A fact of a person's birth notice, such as posted in a newspaper or
    /// other publishing medium.
    BirthNotice,

    /// A fact of an official blessing received by a person, such as at the
    /// hands of a clergy member or at another religious rite.
    Blessing,

    /// A fact of a person's branch within an extended clan.
    Branch,

    /// A fact of the burial of person's body after death.
    Burial,
    Caste,

    /// A fact of a person's participation in a census.
    Census,

    /// A fact of a person's christening *at birth*.
    ///
    /// Note: use [`AdultChristening`](crate::FactType::AdultChristening) for
    /// the christening as an adult.
    Christening,
    Circumcision,
    Clan,

    /// A fact of a person's confirmation (or other rite of initiation) in a
    /// church or religion.
    Confirmation,

    /// A fact of the appearance of a person in a court proceeding.
    Court,

    /// A fact of the cremation of person's body after death.       
    Cremation,
    Death,

    /// A fact of an education or an educational achievement (e.g., diploma,
    /// graduation, scholarship, etc.) of a person.
    Education,

    /// A fact of a person's enrollment in an educational program or
    /// institution.
    EducationEnrollment,
    Emigration,
    Enslavement,
    Ethnicity,

    /// A fact of a person's excommunication from a church.
    Excommunication,

    /// A fact of a person's first communion in a church.
    FirstCommunion,
    Funeral,
    GenderChange,

    /// A fact of a person's generation number, indicating the number of
    /// generations the person is removed from a known "first" ancestor.
    GenerationNumber,

    /// A fact of a person's graduation from a scholastic institution.
    Graduation,

    /// A fact of a person's heimat.
    ///
    /// "Heimat" refers to a person's affiliation by birth to a specific
    /// geographic place. Distinct heimaten are often useful as indicators
    /// that two persons of the same name are not likely to be closely related
    /// genealogically. In English, "heimat" may be described using terms
    /// like "ancestral home", "homeland", or "place of origin".
    Heimat,
    Immigration,
    Imprisonment,

    /// A legal inquest.
    ///
    /// Inquests usually only occur when there’s something suspicious about the
    /// death. Inquests might in some instances lead to a
    /// murder investigation. Most people that die have a death certificate
    /// wherein a doctor indicates the cause of death and often
    /// indicates when the decedent was last seen by that physician; these
    /// require no inquest.
    Inquest,

    /// A fact of a land transaction enacted by a person.
    LandTransaction,

    /// A fact of a language spoken by a person.
    Language,

    /// A fact of a record of a person's living for a specific period.
    ///
    /// This is designed to include "flourish", defined to mean the time period
    /// in an adult's life where he was most productive, perhaps as a writer
    /// or member of the state assembly. It does not reflect the person's birth
    /// and death dates.
    Living,
    MaritalStatus,

    /// A fact of a person's medical record, such as for an illness or hospital
    /// stay.
    Medical,
    MilitaryAward,
    MilitaryDischarge,
    MilitaryDraftRegistration,
    MilitaryInduction,
    MilitaryService,

    /// A fact of a person's church mission.
    Mission,

    /// A fact of a person's move (i.e. change of residence) from a location.
    MoveFrom,

    /// A fact of a person's move (i.e. change of residence) to a new location.
    MoveTo,

    /// A fact that a person was born as part of a multiple birth (e.g. twin,
    /// triplet, etc.)
    MultipleBirth,

    /// A fact of a person's national id (e.g. social security number).
    NationalId,
    Nationality,

    /// A fact of a person's naturalization (i.e. acquisition of citizenship and
    /// nationality).
    Naturalization,

    /// A fact of the number of children of a person or relationship.
    NumberOfChildren, // Also a couple fact type.
    NumberOfMarriages,
    Obituary,

    /// A fact of a person's official (government) position.
    OfficialPosition,

    /// A fact of a person's occupation or employment.
    Occupation,

    /// A fact of a person's ordination to a stewardship in a church.
    Ordination,

    /// A fact of a person's legal pardon.
    Pardon,
    PhysicalDescription,

    /// A fact of a receipt of probate of a person's property.
    Probate,

    /// A fact of a person's property or possessions.
    Property,

    /// A fact of the declaration of a person's race, presumably in a historical
    /// document.
    Race,
    Religion,
    Residence,
    Retirement,
    Stillbirth,
    TaxAssessment,
    Tribe,
    Will,

    /// A fact of a person's visit to a place different from the person's
    /// residence.
    Visit,

    /// A fact of a person's *yahrzeit* date.
    ///
    /// A person's yahzeit is the anniversary of their death as measured by the
    /// Hebrew calendar.
    Yahrzeit,

    // Couple fact types.
    /// The fact of an annulment of a marriage.
    Annulment,
    CommonLawMarriage,
    CivilUnion,
    Divorce,
    DivorceFiling,
    DomesticPartnership,

    /// The fact of an engagement to be married.
    Engagement,
    Marriage,
    MarriageBanns,
    MarriageContract,
    MarriageLicense,
    MarriageNotice,
    Separation,

    // Parent-child fact types.
    /// A fact about an adoptive relationship between a parent an a child.
    AdoptiveParent,

    /// A fact the biological relationship between a parent and a child.
    BiologicalParent,

    /// A fact about the child order between a parent and a child.
    ChildOrder,

    /// A fact about an entering heir relationship between a parent and a child.
    ///
    /// An entering heir is received from another parent as an "exiting heir"
    /// for designation of inheritance.
    EnteringHeir,

    /// A fact about an exiting heir relationship between a parent and a child.
    ///
    /// An exiting heir is given as an "entering heir" to another parent for
    /// designation of inheritance.
    ExitingHeir,

    /// A fact about a foster relationship between a foster parent and a child.
    FosterParent,

    /// A fact about a legal guardianship between a parent and a child.
    GuardianParent,

    /// A fact about a legal guardianship between a parent and a child.
    StepParent,

    /// A fact about a sociological relationship between a parent and a child,
    /// but not definable in typical legal or biological terms.
    SociologicalParent,

    /// A fact about a pregnancy surrogate relationship between a parent and a
    /// child.
    SurrogateParent,

    // Catch all
    Custom(Uri),
}

impl_enumasstring_yaserialize_yadeserialize!(FactType, "FactType");

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
            Self::Custom(c) => write!(f, "{c}"),
        }
    }
}

impl From<EnumAsString> for FactType {
    #[allow(clippy::too_many_lines)]
    fn from(f: EnumAsString) -> Self {
        // If you need to generate this mapping in the future, the easiest way is to
        // copy and paste the tables in https://github.com/FamilySearch/gedcomx/blob/master/specifications/fact-types-specification.md.
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

impl Arbitrary for FactType {
    #[allow(clippy::too_many_lines)]
    fn arbitrary(g: &mut Gen) -> Self {
        let options = vec![
            Self::Adoption,
            Self::AdultChristening,
            Self::Amnesty,
            Self::AncestralHall,
            Self::AncestralPoem,
            Self::Apprenticeship,
            Self::Arrest,
            Self::Award,
            Self::Baptism,
            Self::BarMitzvah,
            Self::BatMitzvah,
            Self::Birth,
            Self::BirthNotice,
            Self::Blessing,
            Self::Branch,
            Self::Burial,
            Self::Caste,
            Self::Census,
            Self::Christening,
            Self::Circumcision,
            Self::Clan,
            Self::Confirmation,
            Self::Court,
            Self::Cremation,
            Self::Death,
            Self::Education,
            Self::EducationEnrollment,
            Self::Emigration,
            Self::Enslavement,
            Self::Ethnicity,
            Self::Excommunication,
            Self::FirstCommunion,
            Self::Funeral,
            Self::GenderChange,
            Self::GenerationNumber,
            Self::Graduation,
            Self::Immigration,
            Self::Imprisonment,
            Self::Inquest,
            Self::LandTransaction,
            Self::Language,
            Self::Living,
            Self::MaritalStatus,
            Self::Medical,
            Self::MilitaryAward,
            Self::MilitaryDischarge,
            Self::MilitaryDraftRegistration,
            Self::MilitaryInduction,
            Self::MilitaryService,
            Self::Mission,
            Self::MoveFrom,
            Self::MoveTo,
            Self::MultipleBirth,
            Self::NationalId,
            Self::Nationality,
            Self::Naturalization,
            Self::NumberOfChildren,
            Self::NumberOfMarriages,
            Self::Obituary,
            Self::OfficialPosition,
            Self::Occupation,
            Self::Ordination,
            Self::Pardon,
            Self::PhysicalDescription,
            Self::Probate,
            Self::Property,
            Self::Race,
            Self::Religion,
            Self::Residence,
            Self::Retirement,
            Self::Stillbirth,
            Self::TaxAssessment,
            Self::Tribe,
            Self::Will,
            Self::Visit,
            Self::Yahrzeit,
            Self::Annulment,
            Self::CommonLawMarriage,
            Self::CivilUnion,
            Self::Divorce,
            Self::DivorceFiling,
            Self::DomesticPartnership,
            Self::Engagement,
            Self::Marriage,
            Self::MarriageBanns,
            Self::MarriageContract,
            Self::MarriageLicense,
            Self::MarriageNotice,
            Self::Separation,
            Self::AdoptiveParent,
            Self::BiologicalParent,
            Self::ChildOrder,
            Self::EnteringHeir,
            Self::ExitingHeir,
            Self::FosterParent,
            Self::GuardianParent,
            Self::StepParent,
            Self::SociologicalParent,
            Self::SurrogateParent,
            Self::Custom(Uri::arbitrary(g)),
        ];

        g.choose(&options).unwrap().clone()
    }
}

/// Fact qualifiers.
#[derive(Debug, PartialEq, Clone, Eq)]
#[non_exhaustive]
pub enum FactQualifier {
    /// The age of a person at the event described by the fact.
    Age,

    /// The cause of the fact, such as the cause of death.
    Cause,

    /// The religion associated with a religious event such as a baptism or
    /// excommunication.
    Religion,

    /// The name of the transport associated with an event that indicates a
    /// move.
    Transport,

    /// An indicator that the event occurred non-consensually, e.g. under
    /// enslavement.
    NonConsensual,
}

impl FromStr for FactQualifier {
    type Err = GedcomxError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "http://gedcomx.org/Age" => Ok(Self::Age),
            "http://gedcomx.org/Cause" => Ok(Self::Cause),
            "http://gedcomx.org/Religion" => Ok(Self::Religion),
            "http://gedcomx.org/Transport" => Ok(Self::Transport),
            "http://gedcomx.org/NonConsensual" => Ok(Self::NonConsensual),
            _ => Err(GedcomxError::QualifierParse {
                parsed_string: s.to_string(),
            }),
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
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::TestData;

    #[test]
    fn json_serialize_custom_fact_type() {
        let t = FactType::Custom("this is a custom fact".into());
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(json, r#""this is a custom fact""#);
    }

    #[test]
    fn json_deserialize_custom_fact_type() {
        let json = r#""this is a custom fact""#;
        let t: FactType = serde_json::from_str(json).unwrap();
        assert_eq!(t, FactType::Custom("this is a custom fact".into()));
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
            },
            "date" : { "original": "date" }
        }"#;

        let fact: Fact = serde_json::from_str(json).unwrap();

        assert_eq!(
            fact,
            Fact {
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
                fact_type: FactType::Birth,
                place: Some(PlaceReference {
                    original: Some("This is a place reference".to_string()),
                    description_ref: Some("D-1".into())
                }),
                value: Some("the original value of the fact".to_string()),
                qualifiers: vec![Qualifier {
                    name: FactQualifier::Age.into(),
                    value: Some("val".into())
                }],
                date: Some(Date::new(Some("date"), None))
            }
        );
    }

    #[test]
    fn xml_deserialize() {
        let xml = "<Fact xmlns=\"http://gedcomx.org/v1/\" type=\"http://gedcomx.org/Award\"><value>Fact value</value><qualifier name=\"http://gedcomx.org/Cause\">Just because</qualifier></Fact>";
        let fact: Fact = yaserde::de::from_str(xml).unwrap();

        assert_eq!(
            fact,
            Fact::builder(FactType::Award)
                .value("Fact value")
                .qualifier(Qualifier::new(FactQualifier::Cause, Some("Just because")))
                .build()
        );
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
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
                fact_type: FactType::Birth,
                place: None,
                value: None,
                qualifiers: vec![],
                date: None
            }
        );
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let fact = Fact {
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
            fact_type: FactType::Birth,
            place: Some(PlaceReference {
                original: Some("This is a place reference".to_string()),
                description_ref: Some("D-1".into()),
            }),
            value: Some("the original value of the fact".to_string()),
            qualifiers: vec![Qualifier {
                name: FactQualifier::Age.into(),
                value: Some("val".into()),
            }],
            date: Some(Date::new(Some("date"), None)),
        };

        let json = serde_json::to_string(&fact).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"type":"http://gedcomx.org/Birth","date":{"original":"date"},"place":{"original":"This is a place reference","description":"D-1"},"value":"the original value of the fact","qualifiers":[{"name":"http://gedcomx.org/Age","value":"val"}]}"#
        );
    }

    #[test]
    fn xml_serialize() {
        let fact = Fact::builder(FactType::Award)
            .value("Fact value")
            .qualifier(Qualifier::new(FactQualifier::Cause, Some("Just because")))
            .build();
        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };
        let xml = yaserde::ser::to_string_with_config(&fact, &config).unwrap();
        assert_eq!(
            xml,
            "<Fact xmlns=\"http://gedcomx.org/v1/\" type=\"http://gedcomx.org/Award\"><value>Fact value</value><qualifier name=\"http://gedcomx.org/Cause\">Just because</qualifier></Fact>"
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let data = TestData::new();

        let fact = Fact {
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
            fact_type: FactType::Birth,
            place: None,
            value: None,
            qualifiers: vec![],
            date: None,
        };

        let json = serde_json::to_string(&fact).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"type":"http://gedcomx.org/Birth"}"#
        );
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: Fact) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: Fact = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: Fact) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: Fact = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

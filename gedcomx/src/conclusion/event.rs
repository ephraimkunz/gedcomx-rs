use crate::{
    Conclusion, ConclusionData, Date, EnumAsString, EventRole, PlaceReference, Subject,
    SubjectData, Uri,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Event {
    #[serde(flatten)]
    pub subject: SubjectData,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub event_type: Option<EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<PlaceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub roles: Vec<EventRole>,
}

impl Event {
    pub fn new(
        subject: SubjectData,
        event_type: Option<EventType>,
        date: Option<Date>,
        place: Option<PlaceReference>,
        roles: Vec<EventRole>,
    ) -> Self {
        Self {
            subject,
            event_type,
            date,
            place,
            roles,
        }
    }

    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }
}

pub struct EventBuilder(Event);

impl EventBuilder {
    pub(crate) fn new() -> Self {
        Self(Event::default())
    }

    subject_builder_functions!(Event);

    pub fn event_type(&mut self, event_type: EventType) -> &mut Self {
        self.0.event_type = Some(event_type);
        self
    }

    pub fn date(&mut self, date: Date) -> &mut Self {
        self.0.date = Some(date);
        self
    }

    pub fn place(&mut self, place: PlaceReference) -> &mut Self {
        self.0.place = Some(place);
        self
    }

    pub fn role(&mut self, role: EventRole) -> &mut Self {
        self.0.roles.push(role);
        self
    }

    pub fn build(&self) -> Event {
        Event::new(
            self.0.subject.clone(),
            self.0.event_type.clone(),
            self.0.date.clone(),
            self.0.place.clone(),
            self.0.roles.clone(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum EventType {
    Adoption,
    AdultChristening,
    Annulment,
    Baptism,
    BarMitzvah,
    BatMitzvah,
    Birth,
    Blessing,
    Burial,
    Census,
    Christening,
    Circumcision,
    Confirmation,
    Cremation,
    Death,
    Divorce,
    DivorceFiling,
    Education,
    Engagement,
    Emigration,
    Excommunication,
    FirstCommunion,
    Funeral,
    Immigration,
    LandTransaction,
    Marriage,
    MilitaryAward,
    MilitaryDischarge,
    Mission,
    MoveFrom,
    MoveTo,
    Naturalization,
    Ordination,
    Retirement,
    Custom(Uri),
}

impl From<EnumAsString> for EventType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Adoption" => Self::Adoption,
            "http://gedcomx.org/AdultChristening" => Self::AdultChristening,
            "http://gedcomx.org/Annulment" => Self::Annulment,
            "http://gedcomx.org/Baptism" => Self::Baptism,
            "http://gedcomx.org/BarMitzvah" => Self::BarMitzvah,
            "http://gedcomx.org/BatMitzvah" => Self::BatMitzvah,
            "http://gedcomx.org/Birth" => Self::Birth,
            "http://gedcomx.org/Blessing" => Self::Blessing,
            "http://gedcomx.org/Burial" => Self::Burial,
            "http://gedcomx.org/Census" => Self::Census,
            "http://gedcomx.org/Christening" => Self::Christening,
            "http://gedcomx.org/Circumcision" => Self::Circumcision,
            "http://gedcomx.org/Confirmation" => Self::Confirmation,
            "http://gedcomx.org/Cremation" => Self::Cremation,
            "http://gedcomx.org/Death" => Self::Death,
            "http://gedcomx.org/Divorce" => Self::Divorce,
            "http://gedcomx.org/DivorceFiling" => Self::DivorceFiling,
            "http://gedcomx.org/Education" => Self::Education,
            "http://gedcomx.org/Engagement" => Self::Engagement,
            "http://gedcomx.org/Emigration" => Self::Emigration,
            "http://gedcomx.org/Excommunication" => Self::Excommunication,
            "http://gedcomx.org/FirstCommunion" => Self::FirstCommunion,
            "http://gedcomx.org/Funeral" => Self::Funeral,
            "http://gedcomx.org/Immigration" => Self::Immigration,
            "http://gedcomx.org/LandTransaction" => Self::LandTransaction,
            "http://gedcomx.org/Marriage" => Self::Marriage,
            "http://gedcomx.org/MilitaryAward" => Self::MilitaryAward,
            "http://gedcomx.org/MilitaryDischarge" => Self::MilitaryDischarge,
            "http://gedcomx.org/Mission" => Self::Mission,
            "http://gedcomx.org/MoveFrom" => Self::MoveFrom,
            "http://gedcomx.org/MoveTo" => Self::MoveTo,
            "http://gedcomx.org/Naturalization" => Self::Naturalization,
            "http://gedcomx.org/Ordination" => Self::Ordination,
            "http://gedcomx.org/Retirement" => Self::Retirement,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Adoption => write!(f, "http://gedcomx.org/Adoption"),
            Self::AdultChristening => write!(f, "http://gedcomx.org/AdultChristening"),
            Self::Annulment => write!(f, "http://gedcomx.org/Annulment"),
            Self::Baptism => write!(f, "http://gedcomx.org/Baptism"),
            Self::BarMitzvah => write!(f, "http://gedcomx.org/BarMitzvah"),
            Self::BatMitzvah => write!(f, "http://gedcomx.org/BatMitzvah"),
            Self::Birth => write!(f, "http://gedcomx.org/Birth"),
            Self::Blessing => write!(f, "http://gedcomx.org/Blessing"),
            Self::Burial => write!(f, "http://gedcomx.org/Burial"),
            Self::Census => write!(f, "http://gedcomx.org/Census"),
            Self::Christening => write!(f, "http://gedcomx.org/Christening"),
            Self::Circumcision => write!(f, "http://gedcomx.org/Circumcision"),
            Self::Confirmation => write!(f, "http://gedcomx.org/Confirmation"),
            Self::Cremation => write!(f, "http://gedcomx.org/Cremation"),
            Self::Death => write!(f, "http://gedcomx.org/Death"),
            Self::Divorce => write!(f, "http://gedcomx.org/Divorce"),
            Self::DivorceFiling => write!(f, "http://gedcomx.org/DivorceFiling"),
            Self::Education => write!(f, "http://gedcomx.org/Education"),
            Self::Engagement => write!(f, "http://gedcomx.org/Engagement"),
            Self::Emigration => write!(f, "http://gedcomx.org/Emigration"),
            Self::Excommunication => write!(f, "http://gedcomx.org/Excommunication"),
            Self::FirstCommunion => write!(f, "http://gedcomx.org/FirstCommunion"),
            Self::Funeral => write!(f, "http://gedcomx.org/Funeral"),
            Self::Immigration => write!(f, "http://gedcomx.org/Immigration"),
            Self::LandTransaction => write!(f, "http://gedcomx.org/LandTransaction"),
            Self::Marriage => write!(f, "http://gedcomx.org/Marriage"),
            Self::MilitaryAward => write!(f, "http://gedcomx.org/MilitaryAward"),
            Self::MilitaryDischarge => write!(f, "http://gedcomx.org/MilitaryDischarge"),
            Self::Mission => write!(f, "http://gedcomx.org/Mission"),
            Self::MoveFrom => write!(f, "http://gedcomx.org/MoveFrom"),
            Self::MoveTo => write!(f, "http://gedcomx.org/MoveTo"),
            Self::Naturalization => write!(f, "http://gedcomx.org/Naturalization"),
            Self::Ordination => write!(f, "http://gedcomx.org/Ordination"),
            Self::Retirement => write!(f, "http://gedcomx.org/Retirement"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

impl Conclusion for Event {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.subject_mut().conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("Event")
    }
}

impl Subject for Event {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }

    fn subject_mut(&mut self) -> &mut SubjectData {
        &mut self.subject
    }
}

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, Date, EnumAsString, EventRole, EvidenceReference, Id, Identifier,
    Lang, Note, PlaceReference, ResourceReference, SourceReference, Uri,
};

/// A description of a historical event.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct Event {
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
    // TODO: Validate this at compile time somehow?
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

    /// Whether this subject is to be constrained as an extracted conclusion.
    #[yaserde(attribute)]
    pub extracted: Option<bool>,

    /// References to other subjects that support this subject.
    ///
    /// If provided, each reference MUST resolve to an instance of subject of
    /// the same type as this instance (e.g., if the subject is an instance of
    /// Person, all of its evidence references must resolve to instances of
    /// Person).
    #[yaserde(prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub evidence: Vec<EvidenceReference>,

    /// References to multimedia resources for this subject, such as photos or
    /// videos, intended to provide additional context or illustration for the
    /// subject and not considered evidence supporting the identity of the
    /// subject or its supporting conclusions.
    ///
    /// Media references SHOULD be ordered by priority such that applications
    /// that wish to display a single media item (such as an image) MAY choose
    /// the first applicable media reference. Note that the SourceReference is
    /// used for multimedia references and therefore MUST resolve to a
    /// SourceDescription of the resource, which in turn provides a reference to
    /// the resource itself.
    #[yaserde(prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub media: Vec<SourceReference>,

    /// A list of identifiers for the subject.
    #[yaserde(rename = "identifier", prefix = "gx")]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        with = "crate::serde_vec_identifier_to_map"
    )]
    pub identifiers: Vec<Identifier>,

    /// The type of the event.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub event_type: Option<EventType>,

    /// The date of the event.
    pub date: Option<Date>,

    /// A reference to the place applicable to this event.
    pub place: Option<PlaceReference>,

    /// Information about how persons participated in the event.
    #[yaserde(rename = "role")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub roles: Vec<EventRole>,
}

impl Event {
    pub fn new(
        id: Option<Id>,
        lang: Option<Lang>,
        sources: Vec<SourceReference>,
        analysis: Option<ResourceReference>,
        notes: Vec<Note>,
        confidence: Option<ConfidenceLevel>,
        attribution: Option<Attribution>,
        extracted: Option<bool>,
        evidence: Vec<EvidenceReference>,
        media: Vec<SourceReference>,
        identifiers: Vec<Identifier>,
        event_type: Option<EventType>,
        date: Option<Date>,
        place: Option<PlaceReference>,
        roles: Vec<EventRole>,
    ) -> Self {
        Self {
            id,
            lang,
            sources,
            analysis,
            notes,
            confidence,
            attribution,
            extracted,
            evidence,
            media,
            identifiers,
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
    subject_builder_functions!(Event);

    pub(crate) fn new() -> Self {
        Self(Event::default())
    }

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
            self.0.id.clone(),
            self.0.lang.clone(),
            self.0.sources.clone(),
            self.0.analysis.clone(),
            self.0.notes.clone(),
            self.0.confidence.clone(),
            self.0.attribution.clone(),
            self.0.extracted.clone(),
            self.0.evidence.clone(),
            self.0.media.clone(),
            self.0.identifiers.clone(),
            self.0.event_type.clone(),
            self.0.date.clone(),
            self.0.place.clone(),
            self.0.roles.clone(),
        )
    }
}

/// Standard event types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum EventType {
    Adoption,
    AdultChristening,

    /// An annulment event of a marriage.
    Annulment,
    Baptism,
    BarMitzvah,
    BatMitzvah,
    Birth,

    /// A an official blessing event, such as at the hands of a clergy member or
    /// at another religious rite.
    Blessing,
    Burial,
    Census,

    /// A christening event *at birth*. Note: use
    /// [`AdultChristening`](crate::EventType::AdultChristening) for a
    /// christening event as an adult.
    Christening,
    Circumcision,

    /// A confirmation event (or other rite of initiation) in a church or
    /// religion.
    Confirmation,

    /// A cremation event after death.
    Cremation,
    Death,
    Divorce,
    DivorceFiling,

    /// A education or an educational achievement event (e.g. diploma,
    /// graduation, scholarship, etc.).
    Education,

    /// An engagement to be married event.
    Engagement,
    Emigration,

    /// An excommunication event from a church.
    Excommunication,
    FirstCommunion,
    Funeral,
    Immigration,
    LandTransaction,
    Marriage,
    MilitaryAward,
    MilitaryDischarge,
    Mission,

    /// An event of a move (i.e. change of residence) from a location.
    MoveFrom,

    /// An event of a move (i.e. change of residence) to a location.
    MoveTo,

    /// A naturalization event (i.e. acquisition of citizenship and
    /// nationality).
    Naturalization,
    Ordination,
    Retirement,
    Custom(Uri),
}

impl_enumasstring_yaserialize_yadeserialize!(EventType, "EventType");

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

impl Default for EventType {
    fn default() -> Self {
        Self::Custom(Uri::default())
    }
}

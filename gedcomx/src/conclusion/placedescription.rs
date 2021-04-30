use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, Date, EvidenceReference, Id, Identifier, Lang, Note,
    ResourceReference, SourceReference, TextValue, Uri,
};

/// Describes the details of a place in terms of its name and possibly its type,
/// time period, and/or a geospatial description -- functioning as a description
/// of a place as a snapshot in time.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlaceDescription {
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

    /// A list of standardized (or normalized), fully-qualified (in terms of
    /// what is known of the applicable jurisdictional hierarchy) names for this
    /// place that are applicable to this description of this place.
    #[yaserde(rename = "name", prefix = "gx")]
    pub names: Vec<TextValue>, // TODO: Must contain at least 1 name.

    /// An implementation-specific uniform resource identifier (URI) used to
    /// identify the type of a place (e.g., address, city, county, province,
    /// state, country, etc.).
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub place_type: Option<Uri>,

    /// An identifier for the place being described.
    ///
    ///  Descriptions that provide the same value for place are interpreted as alternate descriptions of the same place. If provided, MUST NOT use a base URI of http://gedcomx.org/. If provided, the value MAY resolve to an external resource that is application-specific and outside the scope of this specification.
    #[yaserde(prefix = "gx", rename = "place")]
    pub place: Option<ResourceReference>,

    /// A reference to a description of the jurisdiction of this place.	If provided, MUST resolve to an instance of http://gedcomx.org/v1/PlaceDescription.
    // TODO: Enforce through type system?
    #[yaserde(prefix = "gx")]
    pub jurisdiction: Option<ResourceReference>,

    /// Angular distance, in degrees, north or south of the Equator (0.0
    /// degrees).
    ///
    /// If provided, MUST provide longitude also. Values range from −90.0
    /// degrees (south of the equator) to 90.0 degrees (north of the equator).
    /// It is assumed that descriptions that provide the same value for the
    /// place property share identical longitude values.
    // TODO: Enforce longitude also set.
    #[yaserde(prefix = "gx")]
    pub latitude: Option<f64>,

    /// Angular distance, in degrees, east or west of the Prime Meridian (0.0
    /// degrees).
    ///
    ///  If provided, MUST provide latitude also. Values range from −180.0
    /// degrees (west of the Meridian) to 180.0 degrees (east of the Meridian).
    /// It is assumed that descriptions that provide the same value for the
    /// place property share identical latitude values.
    // TODO: enforce through type system.
    #[yaserde(prefix = "gx")]
    pub longitude: Option<f64>,

    /// A description of the time period to which this place description is
    /// relevant.
    #[yaserde(rename = "temporalDescription", prefix = "gx")]
    pub temporal_description: Option<Date>,

    /// A reference to a geospatial description of this place.
    ///
    /// It is RECOMMENDED that this geospatial description resolve to a KML
    /// document.
    // TODO: Enforce through type system?
    #[yaserde(rename = "spatialDescription", prefix = "gx")]
    pub spatial_description: Option<ResourceReference>,
}

impl PlaceDescription {
    #[allow(clippy::too_many_arguments)]
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
        names: Vec<TextValue>,
        place_type: Option<Uri>,
        place: Option<ResourceReference>,
        jurisdiction: Option<ResourceReference>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        temporal_description: Option<Date>,
        spatial_description: Option<ResourceReference>,
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
            names,
            place_type,
            place,
            jurisdiction,
            latitude,
            longitude,
            temporal_description,
            spatial_description,
        }
    }

    pub fn builder() -> PlaceDescriptionBuilder {
        PlaceDescriptionBuilder::new()
    }
}

pub struct PlaceDescriptionBuilder(PlaceDescription);

impl PlaceDescriptionBuilder {
    subject_builder_functions!(PlaceDescription);

    pub(crate) fn new() -> Self {
        Self(PlaceDescription::default())
    }

    pub fn latitude(&mut self, latitude: f64) -> &mut Self {
        self.0.latitude = Some(latitude);
        self
    }

    pub fn longitude(&mut self, longitude: f64) -> &mut Self {
        self.0.longitude = Some(longitude);
        self
    }

    pub fn name<I: Into<TextValue>>(&mut self, name: I) -> &mut Self {
        self.0.names.push(name.into());
        self
    }

    // TODO: Fill out rest of builder functions.

    pub fn build(&self) -> PlaceDescription {
        PlaceDescription::new(
            self.0.id.clone(),
            self.0.lang.clone(),
            self.0.sources.clone(),
            self.0.analysis.clone(),
            self.0.notes.clone(),
            self.0.confidence.clone(),
            self.0.attribution.clone(),
            self.0.extracted,
            self.0.evidence.clone(),
            self.0.media.clone(),
            self.0.identifiers.clone(),
            self.0.names.clone(),
            self.0.place_type.clone(),
            self.0.place.clone(),
            self.0.jurisdiction.clone(),
            self.0.latitude,
            self.0.longitude,
            self.0.temporal_description.clone(),
            self.0.spatial_description.clone(),
        )
    }
}

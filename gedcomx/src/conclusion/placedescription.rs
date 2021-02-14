use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Conclusion, ConclusionData, Date, ResourceReference, Subject, SubjectData, TextValue, Uri,
};

/// Describes the details of a place in terms of its name and possibly its type,
/// time period, and/or a geospatial description -- functioning as a description
/// of a place as a snapshot in time.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlaceDescription {
    #[yaserde(flatten)]
    #[serde(flatten)]
    pub subject: SubjectData,

    /// A list of standardized (or normalized), fully-qualified (in terms of
    /// what is known of the applicable jurisdictional hierarchy) names for this
    /// place that are applicable to this description of this place.
    #[yaserde(rename = "name")]
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
    pub place: Option<ResourceReference>,

    /// A reference to a description of the jurisdiction of this place.	If provided, MUST resolve to an instance of http://gedcomx.org/v1/PlaceDescription.
    // TODO: Enforce through type system?
    pub jurisdiction: Option<ResourceReference>,

    /// Angular distance, in degrees, north or south of the Equator (0.0
    /// degrees).
    ///
    /// If provided, MUST provide longitude also. Values range from −90.0
    /// degrees (south of the equator) to 90.0 degrees (north of the equator).
    /// It is assumed that descriptions that provide the same value for the
    /// place property share identical longitude values.
    // TODO: Enforce longitude also set.
    pub latitude: Option<f64>,

    /// Angular distance, in degrees, east or west of the Prime Meridian (0.0
    /// degrees).
    ///
    ///  If provided, MUST provide latitude also. Values range from −180.0
    /// degrees (west of the Meridian) to 180.0 degrees (east of the Meridian).
    /// It is assumed that descriptions that provide the same value for the
    /// place property share identical latitude values.
    // TODO: enforce through type system.
    pub longitude: Option<f64>,

    /// A description of the time period to which this place description is
    /// relevant.
    #[yaserde(rename = "temporalDescription")]
    pub temporal_description: Option<Date>,

    /// A reference to a geospatial description of this place.
    ///
    /// It is RECOMMENDED that this geospatial description resolve to a KML
    /// document.
    // TODO: Enforce through type system?
    #[yaserde(rename = "spatialDescription")]
    pub spatial_description: Option<ResourceReference>,
}

impl PlaceDescription {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subject: SubjectData,
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
            subject,
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
            self.0.subject.clone(),
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

impl Conclusion for PlaceDescription {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.subject_mut().conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("PlaceDescription")
    }
}

impl Subject for PlaceDescription {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }

    fn subject_mut(&mut self) -> &mut SubjectData {
        &mut self.subject
    }
}

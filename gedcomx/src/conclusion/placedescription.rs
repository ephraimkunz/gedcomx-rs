use crate::{
    Conclusion, ConclusionData, Date, ResourceReference, Subject, SubjectData, TextValue, Uri,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlaceDescription {
    #[serde(flatten)]
    pub subject: SubjectData,

    pub names: Vec<TextValue>, // Must contain at least 1 name.

    #[serde(rename = "type")]
    pub typee: Option<Uri>,

    pub place: Option<ResourceReference>,

    pub jurisdiction: Option<ResourceReference>,

    pub latitude: Option<f64>,

    pub longitude: Option<f64>,

    pub temporal_description: Option<Date>,

    pub spatial_description: Option<ResourceReference>,
}

impl PlaceDescription {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subject: SubjectData,
        names: Vec<TextValue>, // Must contain at least 1 name.
        typee: Option<Uri>,
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
            typee,
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
    pub(crate) fn new() -> Self {
        Self(PlaceDescription::default())
    }

    subject_builder_functions!(PlaceDescription);

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

    pub fn build(&self) -> PlaceDescription {
        PlaceDescription::new(
            self.0.subject.clone(),
            self.0.names.clone(),
            self.0.typee.clone(),
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

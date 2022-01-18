use std::convert::TryInto;

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, Date, EvidenceReference, Id, Identifier, Lang, Note,
    ResourceReference, Result, SourceReference, TextValue, Uri,
};

/// Describes the details of a place in terms of its name and possibly its type,
/// time period, and/or a geospatial description -- functioning as a description
/// of a place as a snapshot in time.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/",
    // Needed so nested deserialization with the same name works, see https://github.com/media-io/yaserde/issues/110.
    rename = "place"
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
    pub names: Vec<TextValue>,

    /// An implementation-specific uniform resource identifier (URI) used to
    /// identify the type of a place (e.g., address, city, county, province,
    /// state, country, etc.).
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub place_type: Option<Uri>,

    /// An identifier for the place being described.
    ///
    /// Descriptions that provide the same value for place are interpreted as alternate descriptions of the same place. If provided, MUST NOT use a base URI of http://gedcomx.org/. If provided, the value MAY resolve to an external resource that is application-specific and outside the scope of this specification.
    #[yaserde(prefix = "gx")]
    pub place: Option<ResourceReference>,

    /// A reference to a description of the jurisdiction of this place.	If provided, MUST resolve to an instance of http://gedcomx.org/v1/PlaceDescription.
    #[yaserde(prefix = "gx")]
    pub jurisdiction: Option<ResourceReference>,

    /// Angular distance, in degrees, north or south of the Equator (0.0
    /// degrees).
    ///
    /// If provided, MUST provide longitude also. Values range from −90.0
    /// degrees (south of the equator) to 90.0 degrees (north of the equator).
    /// It is assumed that descriptions that provide the same value for the
    /// place property share identical latitude values.
    #[yaserde(prefix = "gx")]
    pub latitude: Option<f64>,

    /// Angular distance, in degrees, east or west of the Prime Meridian (0.0
    /// degrees).
    ///
    ///  If provided, MUST provide latitude also. Values range from −180.0
    /// degrees (west of the Meridian) to 180.0 degrees (east of the Meridian).
    /// It is assumed that descriptions that provide the same value for the
    /// place property share identical longitude values.
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

    pub fn builder<I: Into<TextValue>>(name: I) -> PlaceDescriptionBuilder {
        PlaceDescriptionBuilder::new(name)
    }
}

impl Arbitrary for PlaceDescription {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut place_description = Self::builder(TextValue::arbitrary(g))
            .id(Id::arbitrary(g))
            .lang(Lang::arbitrary(g))
            .note(Note::arbitrary(g))
            .confidence(ConfidenceLevel::arbitrary(g))
            .attribution(Attribution::arbitrary(g))
            .extracted(bool::arbitrary(g))
            .identifier(Identifier::arbitrary(g))
            .place_type(Uri::arbitrary(g))
            .place(ResourceReference::arbitrary(g))
            // Comment out for quickcheck tests because otherwise f64 doesn't compare equal.
            // .latitude_and_longitude(f64::arbitrary(g), f64::arbitrary(g))
            .temporal_description(Date::arbitrary(g))
            .spatial_description(ResourceReference::arbitrary(g))
            .build();

        place_description.sources = vec![SourceReference::arbitrary(g)];
        place_description.analysis = Some(ResourceReference::arbitrary(g));
        place_description.evidence = vec![EvidenceReference::arbitrary(g)];
        place_description.media = vec![SourceReference::arbitrary(g)];
        place_description.jurisdiction = Some(ResourceReference::arbitrary(g));

        place_description
    }
}

pub struct PlaceDescriptionBuilder(PlaceDescription);

impl PlaceDescriptionBuilder {
    subject_builder_functions!(PlaceDescription);

    pub(crate) fn new<I: Into<TextValue>>(name: I) -> Self {
        Self(PlaceDescription {
            names: vec![name.into()],
            ..PlaceDescription::default()
        })
    }

    pub fn name<I: Into<TextValue>>(&mut self, name: I) -> &mut Self {
        self.0.names.push(name.into());
        self
    }

    pub fn place_type(&mut self, place_type: Uri) -> &mut Self {
        self.0.place_type = Some(place_type);
        self
    }

    pub fn place(&mut self, place: ResourceReference) -> &mut Self {
        self.0.place = Some(place);
        self
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `jurisdiction` has no `id` set.
    pub fn jurisdiction(&mut self, jurisdiction: &PlaceDescription) -> Result<&mut Self> {
        self.0.jurisdiction = Some(jurisdiction.try_into()?);
        Ok(self)
    }

    pub fn latitude_and_longitude(&mut self, latitude: f64, longitude: f64) -> &mut Self {
        self.0.latitude = Some(latitude);
        self.0.longitude = Some(longitude);
        self
    }

    pub fn temporal_description(&mut self, date: Date) -> &mut Self {
        self.0.temporal_description = Some(date);
        self
    }

    pub fn spatial_description(&mut self, description: ResourceReference) -> &mut Self {
        self.0.spatial_description = Some(description);
        self
    }

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

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r#"{          
            "names" : [ {
              "lang" : "en",
              "value" : "Pope's Creek, Westmoreland, Virginia, United States"
            } ,
            {
              "lang" : "zh",
              "value" : "教皇的小河，威斯特摩兰，弗吉尼亚州，美国"
            } ],
            "type" : "http://identifier/for/the/place/type",
            "place" : { "resource" : "..." },
            "latitude" : 27.9883575,
            "longitude" : 86.9252014,
            "temporalDescription" : { "original": "..." },
            "spatialDescription" : {
              "resource" : "http://uri/for/KML/document"
            }
        }"#;

        let expected_place_description = PlaceDescription::builder(TextValue::new(
            "Pope's Creek, Westmoreland, Virginia, United States",
            Some("en"),
        ))
        .name(TextValue::new(
            "教皇的小河，威斯特摩兰，弗吉尼亚州，美国",
            Some("zh"),
        ))
        .place_type(Uri::from("http://identifier/for/the/place/type"))
        .place(ResourceReference::from("..."))
        .latitude_and_longitude(27.9883575, 86.9252014)
        .temporal_description(Date::new(Some("..."), None))
        .spatial_description(ResourceReference::from("http://uri/for/KML/document"))
        .build();

        let place_description: PlaceDescription = serde_json::from_str(json).unwrap();
        assert_eq!(place_description, expected_place_description);
    }

    #[test]
    fn xml_deserialize() {
        let xml = r#"
        <PlaceDescription type="http://identifier/for/the/place/type">
        <name lang="en">Pope's Creek, Westmoreland, Virginia, United States</name>
        <name lang="zh">教皇的小河，威斯特摩兰，弗吉尼亚州，美国</name>
        <place resource="..."/>
        <latitude>27.9883575</latitude>
        <longitude>86.9252014</longitude>
        <temporalDescription>
          <original>...</original>
        </temporalDescription>
        <spatialDescription resource="http://uri/for/KML/document"/>
      </PlaceDescription>"#;

        let expected_place_description = PlaceDescription::builder(TextValue::new(
            "Pope's Creek, Westmoreland, Virginia, United States",
            Some("en"),
        ))
        .name(TextValue::new(
            "教皇的小河，威斯特摩兰，弗吉尼亚州，美国",
            Some("zh"),
        ))
        .place_type(Uri::from("http://identifier/for/the/place/type"))
        .place(ResourceReference::from("..."))
        .latitude_and_longitude(27.9883575, 86.9252014)
        .temporal_description(Date::new(Some("..."), None))
        .spatial_description(ResourceReference::from("http://uri/for/KML/document"))
        .build();

        let place_description: PlaceDescription = yaserde::de::from_str(xml).unwrap();
        assert_eq!(place_description, expected_place_description);
    }

    #[test]
    fn json_serialize() {
        let place_description = PlaceDescription::builder(TextValue::new(
            "Pope's Creek, Westmoreland, Virginia, United States",
            Some("en"),
        ))
        .name(TextValue::new(
            "教皇的小河，威斯特摩兰，弗吉尼亚州，美国",
            Some("zh"),
        ))
        .place_type(Uri::from("http://identifier/for/the/place/type"))
        .place(ResourceReference::from("..."))
        .latitude_and_longitude(27.9883575, 86.9252014)
        .temporal_description(Date::new(Some("..."), None))
        .spatial_description(ResourceReference::from("http://uri/for/KML/document"))
        .build();

        let json = serde_json::to_string(&place_description).unwrap();

        assert_eq!(
            json,
            r#"{"names":[{"lang":"en","value":"Pope's Creek, Westmoreland, Virginia, United States"},{"lang":"zh","value":"教皇的小河，威斯特摩兰，弗吉尼亚州，美国"}],"type":"http://identifier/for/the/place/type","place":{"resource":"..."},"latitude":27.9883575,"longitude":86.9252014,"temporalDescription":{"original":"..."},"spatialDescription":{"resource":"http://uri/for/KML/document"}}"#
        );
    }

    #[test]
    fn xml_serialize() {
        let place_description = PlaceDescription::builder(TextValue::new(
            "Pope's Creek, Westmoreland, Virginia, United States",
            Some("en"),
        ))
        .name(TextValue::new(
            "教皇的小河，威斯特摩兰，弗吉尼亚州，美国",
            Some("zh"),
        ))
        .place_type(Uri::from("http://identifier/for/the/place/type"))
        .place(ResourceReference::from("..."))
        .latitude_and_longitude(27.9883575, 86.9252014)
        .temporal_description(Date::new(Some("..."), None))
        .spatial_description(ResourceReference::from("http://uri/for/KML/document"))
        .build();

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };

        let xml = yaserde::ser::to_string_with_config(&place_description, &config).unwrap();

        assert_eq!(
            xml,
            r#"<place xmlns="http://gedcomx.org/v1/" type="http://identifier/for/the/place/type"><name xml:lang="en">Pope's Creek, Westmoreland, Virginia, United States</name><name xml:lang="zh">教皇的小河，威斯特摩兰，弗吉尼亚州，美国</name><place resource="..." /><latitude>27.9883575</latitude><longitude>86.9252014</longitude><temporalDescription><original>...</original></temporalDescription><spatialDescription resource="http://uri/for/KML/document" /></place>"#
        );
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: PlaceDescription) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: PlaceDescription = serde_json::from_str(&json).unwrap();
        assert_eq!(input, from_json);
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: PlaceDescription) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: PlaceDescription = yaserde::de::from_str(&xml).unwrap();
        assert_eq!(input, from_xml);
        input == from_xml
    }
}

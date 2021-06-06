use std::vec;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, Date, EvidenceReference, GroupRole, Id, Identifier, Lang, Note,
    PlaceReference, ResourceReference, SourceReference, TextValue,
};

/// A group of of persons.
///
/// The concept of a "group" captures institutional associations between persons
/// that may or may not have direct familial relations between each other.
/// Examples of a group could include plantations, orphanages, or military
/// units.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct Group {
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

    /// A list of names of the group. The list must contain at least 1 name.
    #[yaserde(rename = "name", prefix = "gx")]
    pub names: Vec<TextValue>,

    /// The date of applicability of the group.
    #[yaserde(prefix = "gx")]
    pub date: Option<Date>,

    /// A reference to the place applicable to this group.
    #[yaserde(prefix = "gx")]
    pub place: Option<PlaceReference>,

    /// Information about how persons were associated with the group.
    #[yaserde(rename = "role", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub roles: Vec<GroupRole>,
}

impl Group {
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
        date: Option<Date>,
        place: Option<PlaceReference>,
        roles: Vec<GroupRole>,
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
            date,
            place,
            roles,
        }
    }

    pub fn builder<I: Into<TextValue>>(name: I) -> GroupBuilder {
        GroupBuilder::new(name)
    }
}

pub struct GroupBuilder(Group);

impl GroupBuilder {
    subject_builder_functions!(Group);

    pub(crate) fn new<I: Into<TextValue>>(name: I) -> Self {
        Self(Group {
            names: vec![name.into()],
            ..Group::default()
        })
    }

    pub fn name<I: Into<TextValue>>(&mut self, name: I) -> &mut Self {
        self.0.names.push(name.into());
        self
    }

    pub fn date(&mut self, date: Date) -> &mut Self {
        self.0.date = Some(date);
        self
    }

    pub fn place(&mut self, place_reference: PlaceReference) -> &mut Self {
        self.0.place = Some(place_reference);
        self
    }

    pub fn role(&mut self, role: GroupRole) -> &mut Self {
        self.0.roles.push(role);
        self
    }

    pub fn build(&self) -> Group {
        Group::new(
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
            self.0.date.clone(),
            self.0.place.clone(),
            self.0.roles.clone(),
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
                "value" : "Monticello Plantation"
              } ,
              {
                "lang" : "zh",
                "value" : "monticello种植园"
              } ],
            "date" : { "original": "date" },
            "place" : { "original": "place" },
            "roles" : [ ]
        }"#;

        let group: Group = serde_json::from_str(json).unwrap();

        assert_eq!(
            group,
            Group::builder(TextValue::new("Monticello Plantation", Some("en")))
                .name(TextValue::new("monticello种植园", Some("zh")))
                .date(Date::new(Some("date"), None))
                .place(PlaceReference::new(Some("place"), None))
                .build()
        )
    }

    #[test]
    fn xml_deserialize() {
        let xml = r#"<Group><name lang="en">Monticello Plantation</name><name lang="zh">monticello种植园</name><date><original>date</original></date><place><original>place</original></place></Group>"#;

        let group: Group = yaserde::de::from_str(xml).unwrap();

        assert_eq!(
            group,
            Group::builder(TextValue::new("Monticello Plantation", Some("en")))
                .name(TextValue::new("monticello种植园", Some("zh")))
                .date(Date::new(Some("date"), None))
                .place(PlaceReference::new(Some("place"), None))
                .build()
        )
    }

    #[test]
    fn json_serialize() {
        let group = Group::builder(TextValue::new("Monticello Plantation", Some("en")))
            .name(TextValue::new("monticello种植园", Some("zh")))
            .date(Date::new(Some("date"), None))
            .place(PlaceReference::new(Some("place"), None))
            .build();

        let json = serde_json::to_string(&group).unwrap();

        assert_eq!(
            json,
            r#"{"names":[{"lang":"en","value":"Monticello Plantation"},{"lang":"zh","value":"monticello种植园"}],"date":{"original":"date"},"place":{"original":"place"}}"#
        )
    }

    #[test]
    fn xml_serialize() {
        let group = Group::builder(TextValue::new("Monticello Plantation", Some("en")))
            .name(TextValue::new("monticello种植园", Some("zh")))
            .date(Date::new(Some("date"), None))
            .place(PlaceReference::new(Some("place"), None))
            .build();

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };
        let xml = yaserde::ser::to_string_with_config(&group, &config).unwrap();

        assert_eq!(
            xml,
            r#"<Group xmlns="http://gedcomx.org/v1/"><name xml:lang="en">Monticello Plantation</name><name xml:lang="zh">monticello种植园</name><date><original>date</original></date><place><original>place</original></place></Group>"#
        )
    }
}

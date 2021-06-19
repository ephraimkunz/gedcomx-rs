use std::{convert::TryInto, fmt};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Agent, Attribution, Coverage, Document, EnumAsString, Id, Identifier, Note, ResourceReference,
    Result, SourceCitation, SourceReference, TextValue, Timestamp, Uri,
};

/// A description of a source of genealogical information.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct SourceDescription {
    /// An identifier for the data structure holding the source description
    /// data.
    #[yaserde(attribute)]
    pub id: Option<Id>,

    /// The type of resource being described.
    #[yaserde(rename = "resourceType", attribute)]
    pub resource_type: Option<ResourceType>,

    /// The citation(s) for this source.
    ///
    /// At least one citation MUST be provided. If more than one citation is
    /// provided, citations are assumed to be given in order of preference, with
    /// the most preferred citation in the first position in the list.
    #[yaserde(rename = "citation", prefix = "gx")]
    pub citations: Vec<SourceCitation>,

    /// A hint about the media type of the resource being described.
    ///
    /// If provided, MUST be a valid MIME (media) type as specified by RFC 4288.
    #[yaserde(rename = "mediaType", attribute)]
    pub media_type: Option<String>,

    /// A uniform resource identifier (URI) for the resource being described.
    #[yaserde(attribute)]
    pub about: Option<Uri>,

    /// A reference to the entity that mediates access to the described source.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Agent.
    #[yaserde(prefix = "gx")]
    pub mediator: Option<ResourceReference>,

    /// A reference to the entity responsible for making the described source
    /// available.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Agent.
    #[yaserde(prefix = "gx")]
    pub publisher: Option<ResourceReference>,

    /// A reference to the entities that authored the described source.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Agent.
    #[yaserde(rename = "author", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authors: Vec<ResourceReference>,

    /// A list of references to any sources from which this source is derived.
    #[yaserde(rename = "source", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    /// A reference to a document containing analysis about this source.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Document of type http://gedcomx.org/Analysis.
    #[yaserde(prefix = "gx")]
    pub analysis: Option<ResourceReference>,

    /// A reference to the source that contains this source, i.e. its parent
    /// context. Used when the description of a source is not complete without
    /// the description of its parent (or containing) source.
    #[yaserde(rename = "componentOf", prefix = "gx")]
    pub component_of: Option<SourceReference>,

    /// The display name(s) for this source.
    ///
    /// If more than one title is provided, titles are assumed to be given in
    /// order of preference, with the most preferred title in the first position
    /// in the list.
    #[yaserde(rename = "title", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub titles: Vec<TextValue>,

    /// A list of notes about a source.
    #[yaserde(rename = "note", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub notes: Vec<Note>,

    /// The attribution of this source description.
    ///
    /// If not provided, the attribution of the containing data set (e.g. file)
    /// of the source description is assumed.
    #[yaserde(prefix = "gx")]
    pub attribution: Option<Attribution>,

    /// The rights for this resource.
    #[yaserde(prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub rights: Vec<ResourceReference>,

    /// The coverage of the resource.
    #[yaserde(prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub coverage: Vec<Coverage>,

    /// Human-readable descriptions of this source.
    ///
    /// If more than one description is provided, descriptions are assumed to be
    /// given in order of preference, with the most preferred description in the
    /// first position in the list.
    #[yaserde(rename = "description", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub descriptions: Vec<TextValue>,

    /// A list of identifiers for the resource being described.
    #[yaserde(rename = "identifier", prefix = "gx")]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        with = "crate::serde_vec_identifier_to_map"
    )]
    pub identifiers: Vec<Identifier>,

    /// Timestamp of when the resource being described was created.
    #[yaserde(prefix = "gx")]
    pub created: Option<Timestamp>,

    /// Timestamp of when the resource being described was modified.
    #[yaserde(prefix = "gx")]
    pub modified: Option<Timestamp>,

    /// Timestamp of when the resource being described was published.
    #[yaserde(prefix = "gx")]
    pub published: Option<Timestamp>,

    /// A reference to the repository that contains the described resource.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Agent.
    #[yaserde(prefix = "gx")]
    pub repository: Option<ResourceReference>,
}

#[allow(clippy::similar_names)]
impl SourceDescription {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Option<Id>,
        resource_type: Option<ResourceType>,
        citations: Vec<SourceCitation>,
        media_type: Option<String>,
        about: Option<Uri>,
        mediator: Option<ResourceReference>,
        publisher: Option<ResourceReference>,
        authors: Vec<ResourceReference>,
        sources: Vec<SourceReference>,
        analysis: Option<ResourceReference>,
        component_of: Option<SourceReference>,
        titles: Vec<TextValue>,
        notes: Vec<Note>,
        attribution: Option<Attribution>,
        rights: Vec<ResourceReference>,
        coverage: Vec<Coverage>,
        descriptions: Vec<TextValue>,
        identifiers: Vec<Identifier>,
        created: Option<Timestamp>,
        modified: Option<Timestamp>,
        published: Option<Timestamp>,
        repository: Option<ResourceReference>,
    ) -> Self {
        Self {
            id,
            resource_type,
            citations,
            media_type,
            about,
            mediator,
            publisher,
            authors,
            sources,
            analysis,
            component_of,
            titles,
            notes,
            attribution,
            rights,
            coverage,
            descriptions,
            identifiers,
            created,
            modified,
            published,
            repository,
        }
    }

    pub fn builder(citation: SourceCitation) -> SourceDescriptionBuilder {
        SourceDescriptionBuilder::new(citation)
    }
}

pub struct SourceDescriptionBuilder(SourceDescription);

impl SourceDescriptionBuilder {
    pub(crate) fn new(citation: SourceCitation) -> Self {
        Self(SourceDescription {
            citations: vec![citation],
            ..SourceDescription::default()
        })
    }

    pub fn id<I: Into<Id>>(&mut self, id: I) -> &mut Self {
        self.0.id = Some(id.into());
        self
    }

    pub fn resource_type(&mut self, resource_type: ResourceType) -> &mut Self {
        self.0.resource_type = Some(resource_type);
        self
    }

    pub fn citation(&mut self, source_citation: SourceCitation) -> &mut Self {
        self.0.citations.push(source_citation);
        self
    }

    pub fn media_type<I: Into<String>>(&mut self, media_type: I) -> &mut Self {
        self.0.media_type = Some(media_type.into());
        self
    }

    pub fn about(&mut self, uri: Uri) -> &mut Self {
        self.0.about = Some(uri);
        self
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `mediator` has no `id` set.
    pub fn mediator(&mut self, mediator: &Agent) -> Result<&mut Self> {
        self.0.mediator = Some(mediator.try_into()?);
        Ok(self)
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `publisher` has no `id` set.
    pub fn publisher(&mut self, publisher: &Agent) -> Result<&mut Self> {
        self.0.publisher = Some(publisher.try_into()?);
        Ok(self)
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `author` has no `id` set.
    pub fn author(&mut self, author: &Agent) -> Result<&mut Self> {
        self.0.authors.push(author.try_into()?);
        Ok(self)
    }

    pub fn source(&mut self, source: SourceReference) -> &mut Self {
        self.0.sources.push(source);
        self
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError`](crate::GedcomxError) if a conversion into
    /// [`Document`](crate::Document) fails. This happens if `document` has no
    /// `id` set or has the wrong `document_type`.
    pub fn analysis(&mut self, analysis: &Document) -> Result<&mut Self> {
        self.0.analysis = Some(analysis.try_into()?);
        Ok(self)
    }

    pub fn component_of(&mut self, component_of: SourceReference) -> &mut Self {
        self.0.component_of = Some(component_of);
        self
    }

    pub fn title<I: Into<TextValue>>(&mut self, title: I) -> &mut Self {
        self.0.titles.push(title.into());
        self
    }

    pub fn note(&mut self, note: Note) -> &mut Self {
        self.0.notes.push(note);
        self
    }

    pub fn attribution(&mut self, attribution: Attribution) -> &mut Self {
        self.0.attribution = Some(attribution);
        self
    }

    pub fn right(&mut self, right: Uri) -> &mut Self {
        self.0.rights.push(ResourceReference::new(right));
        self
    }

    pub fn coverage(&mut self, coverage: Coverage) -> &mut Self {
        self.0.coverage.push(coverage);
        self
    }

    pub fn description<I: Into<TextValue>>(&mut self, description: I) -> &mut Self {
        self.0.descriptions.push(description.into());
        self
    }

    pub fn identifier(&mut self, identifier: Identifier) -> &mut Self {
        self.0.identifiers.push(identifier);
        self
    }

    pub fn created(&mut self, created: Timestamp) -> &mut Self {
        self.0.created = Some(created);
        self
    }

    pub fn modified(&mut self, modified: Timestamp) -> &mut Self {
        self.0.modified = Some(modified);
        self
    }

    pub fn published(&mut self, published: Timestamp) -> &mut Self {
        self.0.published = Some(published);
        self
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `repository` has no `id` set.
    pub fn repository(&mut self, repository: &Agent) -> Result<&mut Self> {
        self.0.repository = Some(repository.try_into()?);
        Ok(self)
    }

    pub fn build(&self) -> SourceDescription {
        SourceDescription::new(
            self.0.id.clone(),
            self.0.resource_type.clone(),
            self.0.citations.clone(),
            self.0.media_type.clone(),
            self.0.about.clone(),
            self.0.mediator.clone(),
            self.0.publisher.clone(),
            self.0.authors.clone(),
            self.0.sources.clone(),
            self.0.analysis.clone(),
            self.0.component_of.clone(),
            self.0.titles.clone(),
            self.0.notes.clone(),
            self.0.attribution.clone(),
            self.0.rights.clone(),
            self.0.coverage.clone(),
            self.0.descriptions.clone(),
            self.0.identifiers.clone(),
            self.0.created.clone(),
            self.0.modified.clone(),
            self.0.published.clone(),
            self.0.repository.clone(),
        )
    }
}

/// Standard resource types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum ResourceType {
    /// A collection of genealogical resources. A collection may contain
    /// physical artifacts (such as a collection of books in a library), records
    /// (such as the 1940 U.S. Census), or digital artifacts (such as an online
    /// genealogical application).
    Collection,

    /// A physical artifact, such as a book.
    PhysicalArtifact,

    /// A digital artifact, such as a digital image of a birth certificate or
    /// other record.
    DigitalArtifact,

    /// A historical record, such as a census record or a vital record.
    Record,

    Custom(Uri),
}

impl_enumasstring_yaserialize_yadeserialize!(ResourceType, "ResourceType");

impl From<EnumAsString> for ResourceType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Collection" => Self::Collection,
            "http://gedcomx.org/PhysicalArtifact" => Self::PhysicalArtifact,
            "http://gedcomx.org/DigitalArtifact" => Self::DigitalArtifact,
            "http://gedcomx.org/Record" => Self::Record,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Collection => write!(f, "http://gedcomx.org/Collection"),
            Self::PhysicalArtifact => write!(f, "http://gedcomx.org/PhysicalArtifact"),
            Self::DigitalArtifact => write!(f, "http://gedcomx.org/DigitalArtifact"),
            Self::Record => write!(f, "http://gedcomx.org/Record"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

impl Default for ResourceType {
    fn default() -> Self {
        Self::Custom(Uri::default())
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn json_deserialize() {
        let json = r##"{
            "id" : "local_id",
            "resourceType" : "http://gedcomx.org/PhysicalArtifact",
            "citations" : [ { "value": "citation", "lang": "en"}],
            "mediaType" : "media_type",
            "about" : "about",
            "mediator": {
                "resource": "#agent"
            },
            "publisher": {
                "resource": "#agent"
            }
          }"##;

        let source_description: SourceDescription = serde_json::from_str(json).unwrap();

        let agent = Agent::builder().id("agent").build();
        let expected_source_description =
            SourceDescription::builder(SourceCitation::new("citation", Some("en".into())))
                .id("local_id")
                .resource_type(ResourceType::PhysicalArtifact)
                .media_type("media_type")
                .about("about".into())
                .mediator(&agent)
                .unwrap()
                .publisher(&agent)
                .unwrap()
                .build();

        assert_eq!(source_description, expected_source_description)
    }

    #[test]
    fn xml_deserialize() {
        let xml = r##"<SourceDescription id="local_id" about="about" mediaType="media_type" resourceType="http://gedcomx.org/PhysicalArtifact">
        <citation xml:lang="en">
          <value>citation</value>
        </citation>
        <mediator resource="#agent" />
        <publisher resource="#agent" />
      </SourceDescription>"##;

        let source_description: SourceDescription = yaserde::de::from_str(xml).unwrap();

        let agent = Agent::builder().id("agent").build();
        let expected_source_description =
            SourceDescription::builder(SourceCitation::new("citation", Some("en".into())))
                .id("local_id")
                .resource_type(ResourceType::PhysicalArtifact)
                .media_type("media_type")
                .about("about".into())
                .mediator(&agent)
                .unwrap()
                .publisher(&agent)
                .unwrap()
                .build();

        assert_eq!(source_description, expected_source_description)
    }

    #[test]
    fn json_serialize() {
        let agent = Agent::builder().id("agent").build();

        let source_description =
            SourceDescription::builder(SourceCitation::new("citation", Some("en".into())))
                .id("local_id")
                .resource_type(ResourceType::PhysicalArtifact)
                .media_type("media_type")
                .about("about".into())
                .mediator(&agent)
                .unwrap()
                .publisher(&agent)
                .unwrap()
                .build();

        let json = serde_json::to_string(&source_description).unwrap();

        let expected_json = r##"{"id":"local_id","resourceType":"http://gedcomx.org/PhysicalArtifact","citations":[{"lang":"en","value":"citation"}],"mediaType":"media_type","about":"about","mediator":{"resource":"#agent"},"publisher":{"resource":"#agent"}}"##;

        assert_eq!(json, expected_json)
    }

    #[test]
    fn xml_serialize() {
        let agent = Agent::builder().id("agent").build();
        let source_description =
            SourceDescription::builder(SourceCitation::new("citation", Some("en".into())))
                .id("local_id")
                .resource_type(ResourceType::PhysicalArtifact)
                .media_type("media_type")
                .about("about".into())
                .mediator(&agent)
                .unwrap()
                .publisher(&agent)
                .unwrap()
                .build();

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };

        let xml = yaserde::ser::to_string_with_config(&source_description, &config).unwrap();

        let expected_xml = r##"<SourceDescription xmlns="http://gedcomx.org/v1/" id="local_id" resourceType="http://gedcomx.org/PhysicalArtifact" mediaType="media_type" about="about"><citation xml:lang="en"><value>citation</value></citation><mediator resource="#agent" /><publisher resource="#agent" /></SourceDescription>"##;

        assert_eq!(xml, expected_xml)
    }
}

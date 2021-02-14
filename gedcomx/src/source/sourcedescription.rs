use std::{convert::TryInto, fmt};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Agent, Attribution, Coverage, EnumAsString, Id, Identifier, Note, ResourceReference, Result,
    SourceCitation, SourceReference, TextValue, Timestamp, Uri,
};

/// A description of a source of genealogical information.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
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
    #[yaserde(rename = "citation")]
    pub citations: Vec<SourceCitation>, // TODO: Must have at least one.

    /// A hint about the media type of the resource being described.
    ///
    /// If provided, MUST be a valid MIME (media) type as specified by RFC 4288.
    // TODO: Newtype?
    #[yaserde(rename = "mediaType", attribute)]
    pub media_type: Option<String>,

    /// A uniform resource identifier (URI) for the resource being described.
    #[yaserde(attribute)]
    pub about: Option<Uri>,

    /// A reference to the entity that mediates access to the described source.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Agent.
    // TODO: Enforce
    pub mediator: Option<ResourceReference>,

    /// A reference to the entity responsible for making the described source
    /// available.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Agent.
    // TODO: Enforce
    pub publisher: Option<ResourceReference>,

    /// A reference to the entities that authored the described source.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Agent.
    // TODO: Enforce
    #[yaserde(rename = "author")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub authors: Vec<ResourceReference>,

    /// A list of references to any sources from which this source is derived.
    #[yaserde(rename = "source")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    /// A reference to a document containing analysis about this source.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Document of type http://gedcomx.org/Analysis.
    pub analysis: Option<ResourceReference>,

    /// A reference to the source that contains this source, i.e. its parent
    /// context. Used when the description of a source is not complete without
    /// the description of its parent (or containing) source.
    #[yaserde(rename = "componentOf")]
    pub component_of: Option<SourceReference>,

    /// The display name(s) for this source.
    ///
    /// If more than one title is provided, titles are assumed to be given in
    /// order of preference, with the most preferred title in the first position
    /// in the list.
    #[yaserde(rename = "title")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub titles: Vec<TextValue>,

    /// A list of notes about a source.
    #[yaserde(rename = "note")]
    pub notes: Option<Note>,

    /// The attribution of this source description.
    ///
    /// If not provided, the attribution of the containing data set (e.g. file)
    /// of the source description is assumed.
    pub attribution: Option<Attribution>,

    /// The rights for this resource.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub rights: Vec<ResourceReference>,

    /// The coverage of the resource.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub coverage: Vec<Coverage>,

    /// Human-readable descriptions of this source.
    ///
    /// If more than one description is provided, descriptions are assumed to be
    /// given in order of preference, with the most preferred description in the
    /// first position in the list.
    #[yaserde(rename = "description")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub descriptions: Vec<TextValue>,

    /// A list of identifiers for the resource being described.
    #[yaserde(rename = "identifier")]
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        with = "crate::serde_vec_identifier_to_map"
    )]
    pub identifiers: Vec<Identifier>,

    /// Timestamp of when the resource being described was created.
    pub created: Option<Timestamp>,

    /// Timestamp of when the resource being described was modified.
    pub modified: Option<Timestamp>,

    /// Timestamp of when the resource being described was published.
    pub published: Option<Timestamp>,

    /// A reference to the repository that contains the described resource.
    ///
    /// If provided, MUST resolve to an instance of http://gedcomx.org/v1/Agent.
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
        notes: Option<Note>,
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

    pub fn builder() -> SourceDescriptionBuilder {
        SourceDescriptionBuilder::new()
    }
}

pub struct SourceDescriptionBuilder(SourceDescription);

impl SourceDescriptionBuilder {
    pub(crate) fn new() -> Self {
        Self(SourceDescription {
            ..SourceDescription::default()
        })
    }

    pub fn id<I: Into<Id>>(&mut self, id: I) -> &mut Self {
        self.0.id = Some(id.into());
        self
    }

    pub fn title<I: Into<TextValue>>(&mut self, title: I) -> &mut Self {
        self.0.titles.push(title.into());
        self
    }

    pub fn about(&mut self, uri: Uri) -> &mut Self {
        self.0.about = Some(uri);
        self
    }

    pub fn citation(&mut self, source_citation: SourceCitation) -> &mut Self {
        self.0.citations.push(source_citation);
        self
    }

    pub fn source(&mut self, source: SourceReference) -> &mut Self {
        self.0.sources.push(source);
        self
    }

    pub fn description<I: Into<TextValue>>(&mut self, description: I) -> &mut Self {
        self.0.descriptions.push(description.into());
        self
    }

    pub fn resource_type(&mut self, resource_type: ResourceType) -> &mut Self {
        self.0.resource_type = Some(resource_type);
        self
    }

    pub fn created(&mut self, created: Timestamp) -> &mut Self {
        self.0.created = Some(created);
        self
    }

    pub fn attribution(&mut self, attribution: Attribution) -> &mut Self {
        self.0.attribution = Some(attribution);
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
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone)]
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

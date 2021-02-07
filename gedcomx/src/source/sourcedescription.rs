use crate::{
    Agent, Attribution, Coverage, EnumAsString, Id, Identifier, Note, ResourceReference, Result,
    SourceCitation, SourceReference, TextValue, Timestamp, Uri,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{convert::TryInto, fmt};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct SourceDescription {
    pub id: Option<Id>,

    pub resource_type: Option<ResourceType>,

    pub citations: Vec<SourceCitation>, // Must have at least one.

    pub media_type: Option<String>,

    pub about: Option<Uri>,

    pub mediator: Option<ResourceReference>,

    pub publisher: Option<ResourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<SourceReference>,

    pub analysis: Option<ResourceReference>,

    pub component_of: Option<SourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub titles: Vec<TextValue>,

    pub notes: Option<Note>,

    pub attribution: Option<Attribution>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub rights: Vec<ResourceReference>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub coverage: Vec<Coverage>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub descriptions: Vec<TextValue>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub identifiers: Vec<Identifier>,

    pub created: Option<Timestamp>,

    pub modified: Option<Timestamp>,

    pub published: Option<Timestamp>,

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

    pub fn repository(&mut self, agent: &Agent) -> Result<&mut Self> {
        self.0.repository = Some(agent.try_into()?);
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum ResourceType {
    Collection,
    PhysicalArtifact,
    DigitalArtifact,
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

use std::{convert::TryInto, fmt};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, EnumAsString, EvidenceReference, Fact, GedcomxError, Id,
    Identifier, Lang, Note, Person, ResourceReference, Result, SourceReference, Uri,
};

/// A relationship between two persons.
///
/// Note: When a relationship type implies direction, the relationship is said
/// to be from person1 to person2. For example, in a parent-child relationship,
/// the relationship is said to be "from a parent to a child"; therefore, the
/// person1 property refers to the parent and the person2 property refers to the
/// child.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct Relationship {
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

    /// The type of the relationship.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub relationship_type: Option<RelationshipType>,

    /// Reference to the first person in the relationship.
    ///
    /// MUST resolve to an instance of http://gedcomx.org/v1/Person.
    #[yaserde(prefix = "gx")]
    pub person1: ResourceReference,

    /// Reference to the second person in the relationship.
    ///
    /// MUST resolve to an instance of http://gedcomx.org/v1/Person.
    // TODO: Check with type system.
    #[yaserde(prefix = "gx")]
    pub person2: ResourceReference,

    /// The facts about the relationship.
    #[yaserde(rename = "fact", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub facts: Vec<Fact>,
}

impl Relationship {
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
        relationship_type: Option<RelationshipType>,
        person1: ResourceReference,
        person2: ResourceReference,
        facts: Vec<Fact>,
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
            relationship_type,
            person1,
            person2,
            facts,
        }
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`SourceReference`](crate::SourceReference) fails.
    /// This happens if `source` has no `id` set.
    pub fn source<I: TryInto<SourceReference, Error = GedcomxError>>(
        &mut self,
        source: I,
    ) -> Result<&mut Self> {
        self.sources.push(source.try_into()?);
        Ok(self)
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if either `person1` or `person2` has no `id` set.
    pub fn builder(person1: &Person, person2: &Person) -> Result<RelationshipBuilder> {
        RelationshipBuilder::new(person1, person2)
    }
}

pub struct RelationshipBuilder(Relationship);

impl RelationshipBuilder {
    subject_builder_functions!(Relationship);

    pub(crate) fn new(person1: &Person, person2: &Person) -> Result<Self> {
        Ok(Self(Relationship {
            person1: person1.try_into()?,
            person2: person2.try_into()?,
            ..Relationship::default()
        }))
    }

    pub fn relationship_type(&mut self, relationship_type: RelationshipType) -> &mut Self {
        self.0.relationship_type = Some(relationship_type);
        self
    }

    pub fn fact(&mut self, fact: Fact) -> &mut Self {
        self.0.facts.push(fact);
        self
    }

    pub fn facts(&mut self, facts: Vec<Fact>) -> &mut Self {
        self.0.facts = facts;
        self
    }

    // TODO: Other builder properties.

    pub fn build(&self) -> Relationship {
        Relationship::new(
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
            self.0.relationship_type.clone(),
            self.0.person1.clone(),
            self.0.person2.clone(),
            self.0.facts.clone(),
        )
    }
}

/// Standard relationship types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum RelationshipType {
    /// A relationship from an ancestor to a descendant.
    AncestorDescendant,

    /// A relationship of a pair of persons.
    Couple,

    /// A relationship from an enslaved person to the enslaver or slaveholder of
    /// the person.
    EnslavedBy,

    /// A relationship from a godparent to a person.
    Godparent,

    /// A relationship from a parent to a child.
    ParentChild,

    Custom(Uri),
}

impl_enumasstring_yaserialize_yadeserialize!(RelationshipType, "RelationshipType");

impl From<EnumAsString> for RelationshipType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/AncestorDescendant" => Self::AncestorDescendant,
            "http://gedcomx.org/Couple" => Self::Couple,
            "http://gedcomx.org/EnslavedBy" => Self::EnslavedBy,
            "http://gedcomx.org/Godparent" => Self::Godparent,
            "http://gedcomx.org/ParentChild" => Self::ParentChild,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::AncestorDescendant => write!(f, "http://gedcomx.org/AncestorDescendant"),
            Self::Couple => write!(f, "http://gedcomx.org/Couple"),
            Self::EnslavedBy => write!(f, "http://gedcomx.org/EnslavedBy"),
            Self::Godparent => write!(f, "http://gedcomx.org/Godparent"),
            Self::ParentChild => write!(f, "http://gedcomx.org/ParentChild"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

impl Default for RelationshipType {
    fn default() -> Self {
        Self::Custom(Uri::default())
    }
}

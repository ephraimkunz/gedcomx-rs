use std::{convert::TryInto, fmt};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    Conclusion, ConclusionData, EnumAsString, Fact, GedcomxError, Person, ResourceReference,
    Result, SourceReference, Subject, SubjectData, Uri,
};

/// A relationship between two persons.
///
/// Note: When a relationship type implies direction, the relationship is said
/// to be from person1 to person2. For example, in a parent-child relationship,
/// the relationship is said to be "from a parent to a child"; therefore, the
/// person1 property refers to the parent and the person2 property refers to the
/// child.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[non_exhaustive]
pub struct Relationship {
    #[serde(flatten)]
    pub subject: SubjectData,

    /// The type of the relationship.
    #[serde(rename = "type")]
    pub relationship_type: Option<RelationshipType>,

    /// Reference to the first person in the relationship.
    ///
    /// MUST resolve to an instance of http://gedcomx.org/v1/Person.
    pub person1: ResourceReference,

    /// Reference to the second person in the relationship.
    ///
    /// MUST resolve to an instance of http://gedcomx.org/v1/Person.
    // TODO: Check with type system.
    pub person2: ResourceReference,

    /// The facts about the relationship.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub facts: Vec<Fact>,
}

impl Relationship {
    pub fn new(
        subject: SubjectData,
        relationship_type: Option<RelationshipType>,
        person1: ResourceReference,
        person2: ResourceReference,
        facts: Vec<Fact>,
    ) -> Self {
        Self {
            subject,
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
        self.subject.conclusion.sources.push(source.try_into()?);
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
            self.0.subject.clone(),
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

impl Conclusion for Relationship {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.subject_mut().conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("Relationship")
    }
}

impl Subject for Relationship {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }

    fn subject_mut(&mut self) -> &mut SubjectData {
        &mut self.subject
    }
}

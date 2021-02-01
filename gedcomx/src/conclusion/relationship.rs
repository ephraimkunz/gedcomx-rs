use crate::{
    Conclusion, ConclusionData, EnumAsString, Fact, GedcomxError, Person, ResourceReference,
    Result, SourceReference, Subject, SubjectData, Uri,
};
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, fmt};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[non_exhaustive]
pub struct Relationship {
    #[serde(flatten)]
    pub subject: SubjectData,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub relationship_type: Option<RelationshipType>,

    pub person1: ResourceReference,

    pub person2: ResourceReference,

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
    /// Will return `GedcomxError` if a conversion into `SourceReference` fails.
    /// This happens if the argument we are converting has no Id set.
    pub fn source<I: TryInto<SourceReference, Error = GedcomxError>>(
        &mut self,
        source: I,
    ) -> Result<&mut Self> {
        self.subject.conclusion.sources.push(source.try_into()?);
        Ok(self)
    }

    /// # Errors
    ///
    /// Will return `GedcomxError::NoId` if either of the two people in the relationship does not have an id, which
    /// is required for them to be part of a `Relationship`.
    pub fn builder(person1: &Person, person2: &Person) -> Result<RelationshipBuilder> {
        RelationshipBuilder::new(person1, person2)
    }
}

pub struct RelationshipBuilder(Relationship);

impl RelationshipBuilder {
    pub(crate) fn new(person1: &Person, person2: &Person) -> Result<Self> {
        Ok(Self(Relationship {
            person1: person1.try_into()?,
            person2: person2.try_into()?,
            ..Relationship::default()
        }))
    }

    subject_builder_functions!(Relationship);

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum RelationshipType {
    AncestorDescendant,
    Couple,
    EnslavedBy,
    Godparent,
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

use crate::{
    subject_builder_functions, Conclusion, ConclusionData, Fact, GedcomxError, Gender, Name,
    Result, SourceReference, Subject, SubjectData,
};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
#[non_exhaustive]
pub struct Person {
    #[serde(flatten)]
    pub subject: SubjectData,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub names: Vec<Name>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub facts: Vec<Fact>,
}

impl Conclusion for Person {
    fn conclusion(&self) -> &ConclusionData {
        &self.subject().conclusion
    }
}

impl Subject for Person {
    fn subject(&self) -> &SubjectData {
        &self.subject
    }
}

impl Person {
    pub fn new(
        subject: SubjectData,
        private: Option<bool>,
        gender: Option<Gender>,
        names: Vec<Name>,
        facts: Vec<Fact>,
    ) -> Self {
        Self {
            subject,
            private,
            gender,
            names,
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

    pub fn builder() -> PersonBuilder {
        PersonBuilder::new()
    }
}

pub struct PersonBuilder(Person);

impl PersonBuilder {
    pub(crate) fn new() -> Self {
        Self(Person::default())
    }

    subject_builder_functions!();

    pub fn name<I: Into<Name>>(&mut self, name: I) -> &mut Self {
        self.0.names.push(name.into());
        self
    }

    pub fn names(&mut self, names: Vec<Name>) -> &mut Self {
        self.0.names = names;
        self
    }

    pub fn gender<I: Into<Gender>>(&mut self, gender: I) -> &mut Self {
        self.0.gender = Some(gender.into());
        self
    }

    pub fn fact(&mut self, fact: Fact) -> &mut Self {
        self.0.facts.push(fact);
        self
    }

    pub fn build(&self) -> Person {
        Person::new(
            self.0.subject.clone(),
            self.0.private,
            self.0.gender.clone(),
            self.0.names.clone(),
            self.0.facts.clone(),
        )
    }
}

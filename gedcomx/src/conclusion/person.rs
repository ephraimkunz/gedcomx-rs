use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, EvidenceReference, Fact, GedcomxError, Gender, Id, Identifier,
    Lang, Name, Note, ResourceReference, Result, SourceReference,
};

/// A description of a person.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct Person {
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

    /// Whether this instance of Person has been designated for limited
    /// distribution or display.
    #[yaserde(attribute)]
    pub private: Option<bool>,

    /// The sex of the person as assigned at birth (see [Sex Assignment](https://en.wikipedia.org/wiki/Sex_assignment)).
    #[yaserde(prefix = "gx")]
    pub gender: Option<Gender>,

    /// The names of the person.
    ///
    /// If more than one name is provided, names are assumed to be given in
    /// order of preference, with the most preferred name in the first position
    /// in the list.
    #[yaserde(rename = "name", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub names: Vec<Name>,

    /// The facts of the person.
    #[yaserde(rename = "fact", prefix = "gx")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub facts: Vec<Fact>,
}

impl Person {
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
        private: Option<bool>,
        gender: Option<Gender>,
        names: Vec<Name>,
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
            private,
            gender,
            names,
            facts,
        }
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`SourceReference`](crate::SourceReference) fails.
    /// This happens if `source` has no `id` set.
    // TODO: Do we want methods like this on the actual structs? If so, should this
    // be a macro we an apply to all "subclasses"?
    pub fn source<I: TryInto<SourceReference, Error = GedcomxError>>(
        &mut self,
        source: I,
    ) -> Result<&mut Self> {
        self.sources.push(source.try_into()?);
        Ok(self)
    }

    pub fn builder() -> PersonBuilder {
        PersonBuilder::new()
    }
}

pub struct PersonBuilder(Person);

impl PersonBuilder {
    subject_builder_functions!(Person);

    pub(crate) fn new() -> Self {
        Self(Person::default())
    }

    pub fn private(&mut self, private: bool) -> &mut Self {
        self.0.private = Some(private);
        self
    }

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
            self.0.private,
            self.0.gender.clone(),
            self.0.names.clone(),
            self.0.facts.clone(),
        )
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{NameForm, NameType};

    #[test]
    fn xml_deserialize() {
        let xml = r##"<person extracted="true" id="P-2">
        <source description="#S-4"/>
        <name>
            <nameForm>
                <fullText>Lo Yau</fullText>
            </nameForm>
        </name>
        <name type="http://gedcomx.org/AlsoKnownAs">
            <nameForm>
                <fullText>Young Hong Wong</fullText>
            </nameForm>
        </name>
    </person>"##;

        let expected_person = Person::builder()
            .extracted(true)
            .id("P-2")
            .source_ref(SourceReference::new("#S-4".into(), None, None, vec![]))
            .name(Name::builder(NameForm::builder().full_text("Lo Yau").build()).build())
            .name(
                Name::builder(NameForm::builder().full_text("Young Hong Wong").build())
                    .name_type(NameType::AlsoKnownAs)
                    .build(),
            )
            .build();
        let person: Person = yaserde::de::from_str(xml).unwrap();

        assert_eq!(person, expected_person)
    }
}

mod conclusiondata;
pub use conclusiondata::{ConclusionData, ConfidenceLevel};

mod date;
pub use date::Date;

mod document;
pub use document::{Document, DocumentBuilder, DocumentType};

mod eventrole;
pub use eventrole::{EventRole, EventRoleBuilder, EventRoleType};

mod event;
pub use event::{Event, EventBuilder, EventType};

mod fact;
pub use fact::{Fact, FactBuilder, FactQualifier, FactType};

mod gender;
pub use gender::{Gender, GenderBuilder, GenderType};

mod group;
pub use group::Group;

mod grouprole;
pub use grouprole::{GroupRole, GroupRoleType};

mod identifier;
pub use identifier::{Identifier, IdentifierType};

mod name;
pub use name::{
    Name, NameBuilder, NameForm, NameFormBuilder, NamePart, NamePartBuilder, NamePartQualifier,
    NamePartType, NameType,
};

mod person;
pub use person::{Person, PersonBuilder};

mod placedescription;
pub use placedescription::{PlaceDescription, PlaceDescriptionBuilder};

mod placereference;
pub use placereference::{PlaceReference, PlaceReferenceBuilder};

mod relationship;
pub use relationship::{Relationship, RelationshipBuilder, RelationshipType};

mod subjectdata;
pub use subjectdata::SubjectData;

/// Trait representing a type that is a conclusion.
pub trait Conclusion {
    fn conclusion(&self) -> &ConclusionData;
    fn conclusion_mut(&mut self) -> &mut ConclusionData;
    fn type_name(&self) -> String;
}

/// Trait representing a type that is a subject.
pub trait Subject: Conclusion {
    fn subject(&self) -> &SubjectData;
    fn subject_mut(&mut self) -> &mut SubjectData;
}

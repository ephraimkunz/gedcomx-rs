use std::{convert::TryInto, fmt};

use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::{
    Attribution, ConfidenceLevel, Date, EnumAsString, Id, Lang, Note, Person, ResourceReference,
    Result, SourceReference, Uri,
};

/// A role of a person in a group.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, YaSerialize, YaDeserialize, PartialEq, Clone, Default)]
#[yaserde(
    prefix = "gx",
    default_namespace = "gx",
    namespace = "gx: http://gedcomx.org/v1/"
)]
#[non_exhaustive]
pub struct GroupRole {
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

    /// Reference to the group participant.	MUST resolve to an instance of
    /// [`Person`](crate::Person).
    #[yaserde(prefix = "gx")]
    pub person: ResourceReference,

    /// The date of applicability of the role.
    #[yaserde(prefix = "gx")]
    pub date: Option<Date>,

    /// Details about the role of the participant in the group.
    #[yaserde(prefix = "gx")]
    pub details: Option<String>,

    /// The participant's role.
    #[yaserde(rename = "type", attribute)]
    #[serde(rename = "type")]
    pub group_role_type: Option<GroupRoleType>,
}

impl GroupRole {
    pub fn new(
        id: Option<Id>,
        lang: Option<Lang>,
        sources: Vec<SourceReference>,
        analysis: Option<ResourceReference>,
        notes: Vec<Note>,
        confidence: Option<ConfidenceLevel>,
        attribution: Option<Attribution>,
        person: ResourceReference,
        date: Option<Date>,
        details: Option<String>,
        group_role_type: Option<GroupRoleType>,
    ) -> Self {
        Self {
            id,
            lang,
            sources,
            analysis,
            notes,
            confidence,
            attribution,
            person,
            date,
            details,
            group_role_type,
        }
    }

    /// # Errors
    ///
    /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
    /// conversion into [`ResourceReference`](crate::ResourceReference) fails.
    /// This happens if `person` has no `id` set.
    pub fn builder(person: &Person) -> Result<GroupRoleBuilder> {
        GroupRoleBuilder::new(person)
    }
}

impl Arbitrary for GroupRole {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut group_role = Self::builder(&Person::arbitrary(g))
            .unwrap()
            .id(Id::arbitrary(g))
            .lang(Lang::arbitrary(g))
            .note(Note::arbitrary(g))
            .confidence(ConfidenceLevel::arbitrary(g))
            .attribution(Attribution::arbitrary(g))
            .date(Date::arbitrary(g))
            .details(crate::arbitrary_trimmed(g))
            .group_role_type(GroupRoleType::arbitrary(g))
            .build();

        group_role.analysis = Some(ResourceReference::arbitrary(g));
        group_role.sources = vec![SourceReference::arbitrary(g)];

        group_role
    }
}

pub struct GroupRoleBuilder(GroupRole);

impl GroupRoleBuilder {
    conclusion_builder_functions!(GroupRole);

    pub(crate) fn new(person: &Person) -> Result<Self> {
        Ok(Self(GroupRole {
            person: person.try_into()?,
            ..GroupRole::default()
        }))
    }

    pub fn date(&mut self, date: Date) -> &mut Self {
        self.0.date = Some(date);
        self
    }

    pub fn details<I: Into<String>>(&mut self, details: I) -> &mut Self {
        self.0.details = Some(details.into());
        self
    }

    pub fn group_role_type(&mut self, role_type: GroupRoleType) -> &mut Self {
        self.0.group_role_type = Some(role_type);
        self
    }

    pub fn build(&self) -> GroupRole {
        GroupRole::new(
            self.0.id.clone(),
            self.0.lang.clone(),
            self.0.sources.clone(),
            self.0.analysis.clone(),
            self.0.notes.clone(),
            self.0.confidence.clone(),
            self.0.attribution.clone(),
            self.0.person.clone(),
            self.0.date.clone(),
            self.0.details.clone(),
            self.0.group_role_type.clone(),
        )
    }
}

/// Group role types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum GroupRoleType {
    Custom(Uri),
}

impl_enumasstring_yaserialize_yadeserialize!(GroupRoleType, "GroupRoleType");

impl From<EnumAsString> for GroupRoleType {
    fn from(f: EnumAsString) -> Self {
        Self::Custom(f.0.into())
    }
}

impl fmt::Display for GroupRoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Custom(c) => write!(f, "{c}"),
        }
    }
}

impl Default for GroupRoleType {
    fn default() -> Self {
        Self::Custom(Uri::default())
    }
}

impl Arbitrary for GroupRoleType {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::Custom(Uri::arbitrary(g))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::TestData;

    #[test]
    fn json_deserialize() {
        let data = TestData::new();

        let json = r#"{   
            "person" : {
                "resource" : "http://identifier/for/person/1"
            },
            "type" : "testType",
            "date" : {
                "original" : "the original text"
            },
            "details" : "details",

            "id" : "local_id",
            "lang" : "en",
            "sources" : [ {
                "description" : "SD-1",
                "descriptionId" : "Description id of the target source",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                },
                "qualifiers" : [ { "name" : "http://gedcomx.org/RectangleRegion", "value" : "rectangle region value" } ]          
            }],
            "analysis" : {
              "resource" : "http://identifier/for/analysis/document"
            },
            "notes" : [ {
                "lang" : "en",
                "subject" : "subject",
                "text" : "This is a note",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                }        
            } ],
            "confidence" : "http://gedcomx.org/High",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            }  
        }"#;

        let group_role: GroupRole = serde_json::from_str(json).unwrap();

        assert_eq!(
            group_role,
            GroupRole {
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
                date: Some(Date {
                    original: Some("the original text".to_string()),
                    formal: None
                }),
                group_role_type: Some(GroupRoleType::Custom("testType".into())),
                details: Some("details".to_string()),
                person: ResourceReference::from("http://identifier/for/person/1")
            }
        );
    }

    #[test]
    fn xml_deserialize() {
        let xml = r##"<GroupRole xmlns="http://gedcomx.org/v1/" type="hello"><person resource="#pid" /><date><original>date</original></date><details>details</details></GroupRole>"##;
        let group_role: GroupRole = yaserde::de::from_str(xml).unwrap();

        let person = Person::builder().id("pid").build();

        assert_eq!(
            group_role,
            GroupRole::builder(&person)
                .unwrap()
                .details("details")
                .date(Date::new(Some("date"), None))
                .group_role_type(GroupRoleType::Custom("hello".into()))
                .build()
        );
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let data = TestData::new();

        let json = r#"{   
            "person" : {
                "resource" : "http://identifier/for/person/1"
            },
            
            "id" : "local_id",
            "lang" : "en",
            "sources" : [ {
                "description" : "SD-1",
                "descriptionId" : "Description id of the target source",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                },
                "qualifiers" : [ { "name" : "http://gedcomx.org/RectangleRegion", "value" : "rectangle region value" } ]          
            }],
            "analysis" : {
              "resource" : "http://identifier/for/analysis/document"
            },
            "notes" : [ {
                "lang" : "en",
                "subject" : "subject",
                "text" : "This is a note",
                "attribution" : {
                    "contributor" : {
                    "resource" : "A-1"
                    },
                    "modified" : 1394175600000
                }        
            } ],
            "confidence" : "http://gedcomx.org/High",
            "attribution" : {
                "contributor" : {
                "resource" : "A-1"
                },
                "modified" : 1394175600000
            }  
        }"#;

        let group_role: GroupRole = serde_json::from_str(json).unwrap();

        assert_eq!(
            group_role,
            GroupRole {
                id: data.conclusion_data.id,
                lang: data.conclusion_data.lang,
                sources: data.conclusion_data.sources,
                analysis: data.conclusion_data.analysis,
                notes: data.conclusion_data.notes,
                confidence: data.conclusion_data.confidence,
                attribution: data.conclusion_data.attribution,
                date: None,
                group_role_type: None,
                details: None,
                person: ResourceReference::from("http://identifier/for/person/1")
            }
        );
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let group_role = GroupRole {
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
            date: Some(Date {
                original: Some("the original text".to_string()),
                formal: None,
            }),
            group_role_type: Some(GroupRoleType::Custom("testType".into())),
            details: Some("details".to_string()),
            person: ResourceReference::from("http://identifier/for/person/1"),
        };

        let json = serde_json::to_string(&group_role).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"person":{"resource":"http://identifier/for/person/1"},"date":{"original":"the original text"},"details":"details","type":"testType"}"#
        );
    }

    #[test]
    fn xml_serialize() {
        let person = Person::builder().id("pid").build();

        let group_role = GroupRole::builder(&person)
            .unwrap()
            .details("details")
            .date(Date::new(Some("date"), None))
            .group_role_type(GroupRoleType::Custom("hello".into()))
            .build();

        let config = yaserde::ser::Config {
            write_document_declaration: false,
            ..yaserde::ser::Config::default()
        };
        let xml = yaserde::ser::to_string_with_config(&group_role, &config).unwrap();

        assert_eq!(
            xml,
            r##"<GroupRole xmlns="http://gedcomx.org/v1/" type="hello"><person resource="#pid" /><date><original>date</original></date><details>details</details></GroupRole>"##
        );
    }

    #[test]
    fn json_serialize_optional_fields() {
        let data = TestData::new();

        let group_role = GroupRole {
            id: data.conclusion_data.id,
            lang: data.conclusion_data.lang,
            sources: data.conclusion_data.sources,
            analysis: data.conclusion_data.analysis,
            notes: data.conclusion_data.notes,
            confidence: data.conclusion_data.confidence,
            attribution: data.conclusion_data.attribution,
            date: None,
            group_role_type: None,
            details: None,
            person: ResourceReference::from("http://identifier/for/person/1"),
        };

        let json = serde_json::to_string(&group_role).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"person":{"resource":"http://identifier/for/person/1"}}"#
        );
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_json(input: GroupRole) -> bool {
        let json = serde_json::to_string(&input).unwrap();
        let from_json: GroupRole = serde_json::from_str(&json).unwrap();
        input == from_json
    }

    #[quickcheck_macros::quickcheck]
    fn roundtrip_xml(input: GroupRole) -> bool {
        let xml = yaserde::ser::to_string(&input).unwrap();
        let from_xml: GroupRole = yaserde::de::from_str(&xml).unwrap();
        input == from_xml
    }
}

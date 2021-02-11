use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Conclusion, ConclusionData, Date, EnumAsString, Lang, Qualifier, Uri};

/// A name of a person.
///
/// A Name is intended to represent a single variant of a person's name. This
/// means that nicknames, spelling variations, or other names
/// (often distinguishable by a name type) should be modeled with separate
/// instances of Name.
///
/// The name forms of a name contain alternate representations of the name. A
/// Name MUST contain at least one name form, presumably a representation of the
/// name that is considered proper and well formed in the person's native,
/// historical cultural context. Other name forms MAY be included, which can be
/// used to represent this name in contexts where the native name form is not
/// easily recognized and interpreted. Alternate forms are more likely in
/// situations where conclusions are being analyzed across cultural context
/// boundaries that have both language and writing script differences.
///
/// For example, a Korean name has a native Korean form, but can also have a
/// Chinese form and a Roman/Latin form—three different name forms,
/// but each representing the same name.
///
/// If more than one name form is provided, included name forms are presumed to
/// be given in order of preference, with the most preferred name form in the
/// first position in the list.
///
/// # Examples
/// Consider the following: a Russian person with the birth name "Александр"
/// (rendered as "Alexander" in English and in a Latin script) that also went by
/// this name's common nickname, "Саша" (rendered as "Sasha" in English).
///
/// It is tempting to think that this situation should be modeled with one Name
/// instance that has several alternate NameForms. The model is not designed to
/// be used in this way. Instead, this person's names ought to be modeled such
/// that the birth name and the nickname are modeled as two separate Name
/// instances: one instance for the birth name, and one for the nickname. The
/// type property MAY be provided to distinguish the birth name from the
/// nickname. Each Name instance MAY have two NameForm instances: one with the
/// native form of the name and another with the
// alternate form. Using an informal pseudo code, it might look something like the following:
/// ```text
/// Name1.type=http://gedcomx.org/BirthName
/// Name1.nameForms[0].fullText=Александр
/// Name1.nameForms[1].fullText=Alexander
///
/// Name2.type=http://gedcomx.org/Nickname
/// Name2.nameForms[0].fullText=Саша
/// Name2.nameForms[1].fullText=Sasha
/// ```
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Name {
    #[serde(flatten)]
    pub conclusion: ConclusionData,

    /// The name type.
    #[serde(rename = "type")]
    pub name_type: Option<NameType>,

    /// The name form(s) that best express this name, usually representations
    /// considered proper and well formed in the person's native, historical
    /// cultural context.
    ///
    /// At least one name form MUST be provided. All included name forms SHOULD
    /// be representations of the same name, and NOT variants of
    /// the name (i.e., not nicknames or spelling variations).
    // TODO: Must be non-empty. Enforce with type system? Vec1<>? Or just error from build method?
    pub name_forms: Vec<NameForm>,

    /// The date of applicability of the name.
    pub date: Option<Date>,
}

impl Name {
    pub fn new(
        conclusion: ConclusionData,
        name_type: Option<NameType>,
        name_forms: Vec<NameForm>,
        date: Option<Date>,
    ) -> Self {
        Self {
            conclusion,
            name_type,
            name_forms,
            date,
        }
    }

    pub fn part_for_type(&self, name_type: &NamePartType) -> Option<&str> {
        self.name_forms.get(0)?.parts.iter().find_map(|n| {
            if n.part_type == Some(name_type.clone()) {
                Some(n.value.as_ref())
            } else {
                None
            }
        })
    }

    pub fn builder() -> NameBuilder {
        NameBuilder::new()
    }
}

impl Conclusion for Name {
    fn conclusion(&self) -> &ConclusionData {
        &self.conclusion
    }

    fn conclusion_mut(&mut self) -> &mut ConclusionData {
        &mut self.conclusion
    }

    fn type_name(&self) -> std::string::String {
        String::from("Name")
    }
}

pub struct NameBuilder(Name);

impl NameBuilder {
    conclusion_builder_functions!(Name);

    pub(crate) fn new() -> Self {
        Self(Name::default())
    }

    pub fn name_type(&mut self, name_type: NameType) -> &mut Self {
        self.0.name_type = Some(name_type);
        self
    }

    pub fn name_form(&mut self, name_form: NameForm) -> &mut Self {
        self.0.name_forms.push(name_form);
        self
    }

    pub fn name_forms(&mut self, name_forms: Vec<NameForm>) -> &mut Self {
        self.0.name_forms = name_forms;
        self
    }

    pub fn build(&self) -> Name {
        Name::new(
            self.0.conclusion.clone(),
            self.0.name_type.clone(),
            self.0.name_forms.clone(),
            self.0.date.clone(),
        )
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self {
            name_forms: vec![NameForm {
                full_text: Some(s.to_string()),
                ..NameForm::default()
            }],
            ..Self::default()
        }
    }
}

/// Standard name types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum NameType {
    /// Name given at birth.
    BirthName,

    /// Name accepted at marriage.
    MarriedName,

    /// "Also known as" name.
    AlsoKnownAs,

    /// Nickname.
    Nickname,

    /// Name given at adoption.
    AdoptiveName,

    /// A formal name, usually given to distinguish it from a name more commonly
    /// used.
    FormalName,

    /// A name given at a religious rite or ceremony.
    ReligiousName,
    Custom(Uri),
}

impl From<EnumAsString> for NameType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/BirthName" => Self::BirthName,
            "http://gedcomx.org/MarriedName" => Self::MarriedName,
            "http://gedcomx.org/AlsoKnownAs" => Self::AlsoKnownAs,
            "http://gedcomx.org/Nickname" => Self::Nickname,
            "http://gedcomx.org/AdoptiveName" => Self::AdoptiveName,
            "http://gedcomx.org/FormalName" => Self::FormalName,
            "http://gedcomx.org/ReligiousName" => Self::ReligiousName,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for NameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::BirthName => write!(f, "http://gedcomx.org/BirthName"),
            Self::MarriedName => write!(f, "http://gedcomx.org/MarriedName"),
            Self::AlsoKnownAs => write!(f, "http://gedcomx.org/AlsoKnownAs"),
            Self::Nickname => write!(f, "http://gedcomx.org/Nickname"),
            Self::AdoptiveName => write!(f, "http://gedcomx.org/AdoptiveName"),
            Self::FormalName => write!(f, "http://gedcomx.org/FormalName"),
            Self::ReligiousName => write!(f, "http://gedcomx.org/ReligiousName"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

/// A representation of a name (a "name form") within a given cultural context,
/// such as a given language and script.
///
/// As names are captured (both in records or in applications), the terms in the
/// name are sometimes classified by type. For example, a certificate of death
/// might prompt for "given name(s)" and "surname". The parts list can be used
/// to represent the terms in the name that have been classified.
///
/// If both a full rendering of the name and a list of parts are provided, it
/// NOT REQUIRED that every term in the fully rendered name appear in the list
/// of parts.
///
/// Name parts in the parts list SHOULD be ordered in the natural order they
/// would be used in the applicable cultural context.
///
/// If a full rendering of the name is not provided (i.e., the name has only
/// been expressed in parts), a full rendering of the name MAY be derived (sans
/// punctuation) by concatenating, in order, each name part value in the list of
/// parts, separating each part with the name part separator appropriate for the
/// applicable cultural context.
///
/// # Examples
/// Consider the following: the Russian name "Пётр Ильи́ч Чайко́вский" in the
/// Cyrillic script, its Latin-script equivalent "Pyotr Ilyich Tchaikovsky", and
/// its anglicised equivalent "Peter Ilyich Tchaikovsky". Using an informal
/// pseudo code, these name forms might be modeled as follows:
/// ```text
/// NameForm1.locale=ru-Cyrl
/// NameForm1.fullText=Пётр Ильи́ч Чайко́вский
/// NameForm1.parts[0].type=http://gedcomx.org/Given
/// NameForm1.parts[0].value=Пётр
/// NameForm1.parts[0].qualifiers[0]=http://gedcomx.org/First
/// NameForm1.parts[1].type=http://gedcomx.org/Middle
/// NameForm1.parts[1].value=Ильи́ч
/// NameForm1.parts[1].qualifiers[0]=http://gedcomx.org/Middle
/// NameForm1.parts[2].type=http://gedcomx.org/Surname
/// NameForm1.parts[2].value=Чайко́вский
///
/// NameForm2.locale=ru-Latn
/// NameForm2.fullText=Pyotr Ilyich Tchaikovsky
/// NameForm2.parts[0].type=http://gedcomx.org/Given
/// NameForm2.parts[0].value=Pyotr
/// NameForm2.parts[0].qualifiers[0]=http://gedcomx.org/First
/// NameForm2.parts[1].type=http://gedcomx.org/Given
/// NameForm2.parts[1].value=Ilyich
/// NameForm2.parts[1].qualifiers[0]=http://gedcomx.org/Middle
/// NameForm2.parts[2].type=http://gedcomx.org/Surname
/// NameForm2.parts[2].value=Tchaikovsky
///
/// NameForm3.locale=en-Latn
/// NameForm3.fullText=Peter Ilyich Tchaikovsky
/// NameForm3.parts[0].type=http://gedcomx.org/Given
/// NameForm3.parts[0].value=Peter
/// NameForm3.parts[0].qualifiers[0]=http://gedcomx.org/First
/// NameForm3.parts[1].type=http://gedcomx.org/Given
/// NameForm3.parts[1].value=Ilyich
/// NameForm3.parts[1].qualifiers[0]=http://gedcomx.org/Middle
/// NameForm3.parts[2].type=http://gedcomx.org/Surname
/// NameForm3.parts[2].value=Tchaikovsky
/// ```
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct NameForm {
    /// The locale identifier for the name form.
    pub lang: Option<Lang>,

    /// A full rendering of the name (or as much of the name as is known).
    pub full_text: Option<String>,

    /// Any identified name parts from the name.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parts: Vec<NamePart>,
}

impl NameForm {
    pub fn new(lang: Option<Lang>, full_text: Option<String>, parts: Vec<NamePart>) -> Self {
        Self {
            lang,
            full_text,
            parts,
        }
    }

    pub fn builder() -> NameFormBuilder {
        NameFormBuilder::new()
    }
}

pub struct NameFormBuilder(NameForm);

impl NameFormBuilder {
    pub(crate) fn new() -> Self {
        Self(NameForm::default())
    }

    pub fn full_text<I: Into<String>>(&mut self, full_text: I) -> &mut Self {
        self.0.full_text = Some(full_text.into());
        self
    }

    pub fn parts(&mut self, parts: Vec<NamePart>) -> &mut Self {
        self.0.parts = parts;
        self
    }

    pub fn part(&mut self, part: NamePart) -> &mut Self {
        self.0.parts.push(part);
        self
    }

    pub fn lang<I: Into<Lang>>(&mut self, lang: I) -> &mut Self {
        self.0.lang = Some(lang.into());
        self
    }

    pub fn build(&self) -> NameForm {
        NameForm::new(
            self.0.lang.clone(),
            self.0.full_text.clone(),
            self.0.parts.clone(),
        )
    }
}

/// A portion of a full name, including the terms that make up that portion.
///
/// Some name parts may have qualifiers to provide additional semantic meaning
/// to the name part (e.g., "given name" or "surname").
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct NamePart {
    /// The type of the name part.
    #[serde(rename = "type")]
    pub part_type: Option<NamePartType>,

    /// The term(s) from the name that make up this name part.
    ///
    /// A name part value MAY contain more than one term from the full name,
    /// such as in the name part "John Fitzgerald" from the full name "John
    /// Fitzgerald Kennedy". If multiple terms are detailed in a single
    /// NamePart, these terms SHOULD be separated using the name separator
    /// appropriate to the locale applicable to the containing name form.
    pub value: String,

    /// Qualifiers to add additional semantic meaning to the name part.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub qualifiers: Vec<Qualifier>,
}

impl NamePart {
    pub fn new(part_type: Option<NamePartType>, value: String, qualifiers: Vec<Qualifier>) -> Self {
        Self {
            part_type,
            value,
            qualifiers,
        }
    }

    pub fn builder<I: Into<String>>(value: I) -> NamePartBuilder {
        NamePartBuilder::new(value)
    }
}

pub struct NamePartBuilder(NamePart);

impl NamePartBuilder {
    pub(crate) fn new<I: Into<String>>(value: I) -> Self {
        Self(NamePart {
            value: value.into(),
            ..NamePart::default()
        })
    }

    pub fn part_type(&mut self, part_type: NamePartType) -> &mut Self {
        self.0.part_type = Some(part_type);
        self
    }

    pub fn qualifier(&mut self, qualifier: Qualifier) -> &mut Self {
        self.0.qualifiers.push(qualifier);
        self
    }

    pub fn build(&self) -> NamePart {
        NamePart::new(
            self.0.part_type.clone(),
            self.0.value.clone(),
            self.0.qualifiers.clone(),
        )
    }
}

/// Standard name part types.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum NamePartType {
    /// A name prefix.
    Prefix,

    /// A name suffix.
    Suffix,

    /// A given name.
    Given,

    /// A surname.
    Surname,
    Custom(Uri),
}

impl From<EnumAsString> for NamePartType {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Prefix" => Self::Prefix,
            "http://gedcomx.org/Suffix" => Self::Suffix,
            "http://gedcomx.org/Given" => Self::Given,
            "http://gedcomx.org/Surname" => Self::Surname,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for NamePartType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Prefix => write!(f, "http://gedcomx.org/Prefix"),
            Self::Suffix => write!(f, "http://gedcomx.org/Suffix"),
            Self::Given => write!(f, "http://gedcomx.org/Given"),
            Self::Surname => write!(f, "http://gedcomx.org/Surname"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

/// Name part qualifiers.
///
/// Identify how the name part was used by the person to which the name applies.
/// For example, a name part qualifier may specify that a given name part was
/// used by the person as a Title.
// TODO: How to include the optional value parameter (or prevent it)? Maybe Into<Qualifier>? https://github.com/FamilySearch/gedcomx/blob/master/specifications/name-part-qualifiers-specification.md#2-name-part-qualifiers
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
#[serde(from = "EnumAsString", into = "EnumAsString")]
pub enum NamePartQualifier {
    /// A designation for honorifics (e.g. Dr., Rev., His Majesty, Haji), ranks
    /// (e.g. Colonel, General, Knight, Esquire), positions (e.g. Count, Chief,
    /// Father, King) or other titles (e.g., PhD, MD).
    Title,

    /// A designation for the name of most prominent in importance among the
    /// names of that type (e.g., the primary given name).
    Primary,

    /// A designation for a name that is not primary in its importance among the
    /// names of that type (e.g., a secondary given name).
    Secondary,

    /// A designation useful for cultures that designate a middle name that is
    /// distinct from a given name and a surname.
    Middle,

    /// A designation for one's familiar name.
    Familiar,

    /// A designation for a name given for religious purposes.
    Religious,

    /// A name that associates a person with a group, such as a clan, tribe, or
    /// patriarchal hierarchy.
    Family,

    /// A designation given by women to their original surname after they adopt
    /// a new surname upon marriage.
    Maiden,

    /// A name derived from a father or paternal ancestor.
    Patronymic,

    /// A name derived from a mother or maternal ancestor.
    Matronymic,

    /// A name derived from associated geography.
    Geographic,

    /// A name derived from one's occupation.
    Occupational,

    /// A name derived from a characteristic.
    Characteristic,

    /// A name mandated by law for populations from Congo Free State / Belgian
    /// Congo / Congo / Democratic Republic of Congo (formerly Zaire).
    Postnom,

    /// A grammatical designation for articles (a, the, dem, las, el, etc.),
    /// prepositions (of, from, aus, zu, op, etc.), initials, annotations (e.g.
    /// twin, wife of, infant, unknown), comparators (e.g. Junior, Senior,
    /// younger, little), ordinals (e.g. III, eighth), descendancy words (e.g.
    /// ben, ibn, bat, bin, bint, bar), and conjunctions (e.g. and, or, nee, ou,
    /// y, o, ne, &).
    Particle,

    /// The "root" of a name part as distinguished from prefixes or suffixes.
    /// For example, the root of the Polish name "Wilkówna" is "Wilk". A
    /// RootName qualifier MUST provide a value property. TODO: Provide
    /// value as associated data?
    RootName,
    Custom(Uri),
}

impl From<EnumAsString> for NamePartQualifier {
    fn from(f: EnumAsString) -> Self {
        match f.0.as_ref() {
            "http://gedcomx.org/Title" => Self::Title,
            "http://gedcomx.org/Primary" => Self::Primary,
            "http://gedcomx.org/Secondary" => Self::Secondary,
            "http://gedcomx.org/Middle" => Self::Middle,
            "http://gedcomx.org/Familiar" => Self::Familiar,
            "http://gedcomx.org/Religious" => Self::Religious,
            "http://gedcomx.org/Family" => Self::Family,
            "http://gedcomx.org/Maiden" => Self::Maiden,
            "http://gedcomx.org/Patronymic" => Self::Patronymic,
            "http://gedcomx.org/Matronymic" => Self::Matronymic,
            "http://gedcomx.org/Geographic" => Self::Geographic,
            "http://gedcomx.org/Occupational" => Self::Occupational,
            "http://gedcomx.org/Characteristic" => Self::Characteristic,
            "http://gedcomx.org/Postnom" => Self::Postnom,
            "http://gedcomx.org/Particle" => Self::Particle,
            "http://gedcomx.org/RootName" => Self::RootName,
            _ => Self::Custom(f.0.into()),
        }
    }
}

impl fmt::Display for NamePartQualifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Title => write!(f, "http://gedcomx.org/Title"),
            Self::Primary => write!(f, "http://gedcomx.org/Primary"),
            Self::Secondary => write!(f, "http://gedcomx.org/Secondary"),
            Self::Middle => write!(f, "http://gedcomx.org/Middle"),
            Self::Familiar => write!(f, "http://gedcomx.org/Familiar"),
            Self::Religious => write!(f, "http://gedcomx.org/Religious"),
            Self::Family => write!(f, "http://gedcomx.org/Family"),
            Self::Maiden => write!(f, "http://gedcomx.org/Maiden"),
            Self::Patronymic => write!(f, "http://gedcomx.org/Patronymic"),
            Self::Matronymic => write!(f, "http://gedcomx.org/Matronymic"),
            Self::Geographic => write!(f, "http://gedcomx.org/Geographic"),
            Self::Occupational => write!(f, "http://gedcomx.org/Occupational"),
            Self::Characteristic => write!(f, "http://gedcomx.org/Characteristic"),
            Self::Postnom => write!(f, "http://gedcomx.org/Postnom"),
            Self::Particle => write!(f, "http://gedcomx.org/Particle"),
            Self::RootName => write!(f, "http://gedcomx.org/RootName"),
            Self::Custom(c) => write!(f, "{}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TestData;

    #[test]
    fn json_deserialize() {
        let data = TestData::new();

        let json = r#"{              
            "type" : "http://gedcomx.org/BirthName",
            "nameForms" : [ {
                "lang" : "en",
                "fullText" : "full text of the name form",
                "parts" : [ {
                    "type" : "http://gedcomx.org/Surname",
                    "value" : "value of the name part",
                    "qualifiers" : [ { "name" : "http://gedcomx.org/Family" }, { "name" : "http://gedcomx.org/Patronymic" } ]                  
                }]                
            }],

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

        let name: Name = serde_json::from_str(json).unwrap();

        assert_eq!(
            name,
            Name {
                conclusion: data.conclusion_data,
                name_type: Some(NameType::BirthName),
                date: None, // TODO: Add in once we get the date type working
                name_forms: vec![NameForm {
                    lang: Some("en".into()),
                    full_text: Some("full text of the name form".to_string()),
                    parts: vec![NamePart {
                        part_type: Some(NamePartType::Surname),
                        value: "value of the name part".to_string(),
                        qualifiers: vec![
                            Qualifier {
                                name: NamePartQualifier::Family.into(),
                                value: None
                            },
                            Qualifier {
                                name: NamePartQualifier::Patronymic.into(),
                                value: None
                            }
                        ]
                    }]
                }]
            }
        )
    }

    #[test]
    fn json_deserialize_optional_fields() {
        let data = TestData::new();

        let json = r#"{              
            "nameForms" : [ {            
            }],

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

        let name: Name = serde_json::from_str(json).unwrap();

        assert_eq!(
            name,
            Name {
                conclusion: data.conclusion_data,
                name_type: None,
                date: None, // TODO: Add in once we get the date type working
                name_forms: vec![NameForm {
                    lang: None,
                    full_text: None,
                    parts: vec![]
                }]
            }
        )
    }

    #[test]
    fn json_serialize() {
        let data = TestData::new();

        let name = Name {
            conclusion: data.conclusion_data,
            name_type: Some(NameType::BirthName),
            date: None, // TODO: Add in once we get the date type working
            name_forms: vec![NameForm {
                lang: Some("en".into()),
                full_text: Some("full text of the name form".to_string()),
                parts: vec![NamePart {
                    part_type: Some(NamePartType::Surname),
                    value: "value of the name part".to_string(),
                    qualifiers: vec![
                        Qualifier {
                            name: NamePartQualifier::Family.into(),
                            value: None,
                        },
                        Qualifier {
                            name: NamePartQualifier::Patronymic.into(),
                            value: None,
                        },
                    ],
                }],
            }],
        };

        let json = serde_json::to_string(&name).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"type":"http://gedcomx.org/BirthName","nameForms":[{"lang":"en","fullText":"full text of the name form","parts":[{"type":"http://gedcomx.org/Surname","value":"value of the name part","qualifiers":[{"name":"http://gedcomx.org/Family"},{"name":"http://gedcomx.org/Patronymic"}]}]}]}"#
        )
    }

    #[test]
    fn json_serialize_optional_fields() {
        let data = TestData::new();

        let name = Name {
            conclusion: data.conclusion_data,
            name_type: None,
            date: None,
            name_forms: vec![NameForm {
                lang: None,
                full_text: None,
                parts: vec![],
            }],
        };

        let json = serde_json::to_string(&name).unwrap();

        assert_eq!(
            json,
            r#"{"id":"local_id","lang":"en","sources":[{"description":"SD-1","descriptionId":"Description id of the target source","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"qualifiers":[{"name":"http://gedcomx.org/RectangleRegion","value":"rectangle region value"}]}],"analysis":{"resource":"http://identifier/for/analysis/document"},"notes":[{"lang":"en","subject":"subject","text":"This is a note","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000}}],"confidence":"http://gedcomx.org/High","attribution":{"contributor":{"resource":"A-1"},"modified":1394175600000},"nameForms":[{}]}"#
        )
    }
}

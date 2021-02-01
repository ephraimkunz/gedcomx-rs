use gedcomx::{
    Agent, Attribution, Date, Fact, FactType, Gedcomx, GenderType, Name, NameForm, NamePart,
    NamePartType, Person, PlaceDescription, PlaceReference, Relationship, SourceCitation,
    SourceDescription, TextValue, Uri,
};

mod common;

fn test_struct() -> Gedcomx {
    let popes_creek = create_popes_creek();
    let mount_vernon = create_mount_vernon();
    let chestnut_grove = create_chestnut_grove();
    let mut george = create_george(&popes_creek, &mount_vernon);
    let mut martha = create_martha(&chestnut_grove, &mount_vernon);
    let mut marriage = create_marriage(&george, &martha);
    let sources = cite_george_martha_and_marriage(&mut george, &mut martha, &mut marriage);
    let contributor = create_contributor();
    let attribution = Attribution::builder()
        .contributor(&contributor)
        .unwrap()
        .build();

    let mut gx = Gedcomx::builder();
    gx.persons(vec![george, martha]);
    gx.relationships(vec![marriage]);
    gx.source_descriptions(sources);
    gx.agents(vec![contributor]);
    gx.attribution(attribution);
    gx.places(vec![popes_creek, mount_vernon, chestnut_grove]);
    gx.build()
}

fn create_popes_creek() -> PlaceDescription {
    PlaceDescription::builder()
        .id("888")
        .latitude(38.192353)
        .longitude(-76.904069)
        .name(TextValue::new(
            "Pope's Creek, Westmoreland, Virginia, United States",
            None,
        ))
        .build()
}

fn create_mount_vernon() -> PlaceDescription {
    PlaceDescription::builder()
        .id("999")
        .latitude(38.721144)
        .longitude(-77.109461)
        .name(TextValue::new(
            "Mount Vernon, Fairfax County, Virginia, United States",
            None,
        ))
        .build()
}

fn create_chestnut_grove() -> PlaceDescription {
    PlaceDescription::builder()
        .id("KKK")
        .latitude(37.518304)
        .longitude(-76.984148)
        .name(TextValue::new(
            "Chestnut Grove, New Kent, Virginia, United States",
            None,
        ))
        .build()
}

fn create_contributor() -> Agent {
    Agent::builder()
        .id("GGG-GGGG")
        .name(TextValue::new("Ryan Heaton", None))
        .build()
}

fn create_george(birth_place: &PlaceDescription, death_place: &PlaceDescription) -> Person {
    let mut person = Person::builder();
    person.gender(GenderType::Male);

    let date = Date::new(Some("February 22, 1732"), Some("+1732-02-22"));
    let place = PlaceReference::builder()
        .original(birth_place.names[0].value.to_lowercase())
        .description_ref(birth_place)
        .unwrap()
        .build();
    let fact = Fact::builder(FactType::Birth)
        .id("123")
        .date(date)
        .place(place)
        .build();

    person.fact(fact);

    let date = Date::new(Some("December 14, 1799"), Some("+1799-12-14T22:00:00"));
    let place = PlaceReference::builder()
        .original(death_place.names[0].value.to_lowercase())
        .description_ref(death_place)
        .unwrap()
        .build();
    let fact = Fact::builder(FactType::Death)
        .id("456")
        .date(date)
        .place(place)
        .build();

    person.fact(fact);

    let mut name = Name::builder();
    let mut name_form = NameForm::builder();
    name_form.full_text("George Washington");
    let mut parts = vec![];

    let part = NamePart::builder("George")
        .part_type(NamePartType::Given)
        .build();
    parts.push(part);

    let part = NamePart::builder("Washington")
        .part_type(NamePartType::Surname)
        .build();
    parts.push(part);

    name_form.parts(parts);
    name.name_form(name_form.build());
    name.id("789");
    let names = vec![name.build()];
    person.names(names);

    person.id("BBB-BBBB");
    person.build()
}

fn create_martha(birth_place: &PlaceDescription, death_place: &PlaceDescription) -> Person {
    let mut person = Person::builder();
    person.gender(GenderType::Male);

    let date = Date::new(Some("June 2, 1731"), Some("+1731-06-02"));
    let place = PlaceReference::builder()
        .original(birth_place.names[0].value.to_lowercase())
        .description_ref(birth_place)
        .unwrap()
        .build();
    let fact = Fact::builder(FactType::Birth)
        .id("321")
        .date(date)
        .place(place)
        .build();

    person.fact(fact);

    let date = Date::new(Some("May 22, 1802"), Some("+1802-05-22"));
    let place = PlaceReference::builder()
        .original(death_place.names[0].value.to_lowercase())
        .description_ref(death_place)
        .unwrap()
        .build();
    let fact = Fact::builder(FactType::Death)
        .id("654")
        .date(date)
        .place(place)
        .build();

    person.fact(fact);

    let mut name = Name::builder();
    let mut name_form = NameForm::builder();
    name_form.full_text("Martha Dandridge Custis");
    let mut parts = vec![];
    let part = NamePart::builder("Martha Dandridge")
        .part_type(NamePartType::Given)
        .build();
    parts.push(part);

    let part = NamePart::builder("Custis")
        .part_type(NamePartType::Surname)
        .build();
    parts.push(part);

    name_form.parts(parts);
    name.name_forms(vec![name_form.build()]);
    name.id("987");
    person.names(vec![name.build()]);

    person.id("CCC-CCCC");

    person.build()
}

fn create_marriage(george: &Person, martha: &Person) -> Relationship {
    let mut relationship = Relationship::builder(george, martha).unwrap();
    relationship.id("DDD-DDDD");

    let date = Date::new(Some("January 6, 1759"), Some("+1759-01-06"));
    let place = PlaceReference::builder()
        .original("White House Plantation")
        .build();
    let marriage = Fact::builder(FactType::Marriage)
        .date(date)
        .place(place)
        .build();

    relationship.facts(vec![marriage]);
    relationship.build()
}

fn cite_george_martha_and_marriage(
    george: &mut Person,
    martha: &mut Person,
    relationship: &mut Relationship,
) -> Vec<SourceDescription> {
    let mut george_source = SourceDescription::builder();
    george_source.id("EEE-EEEE");
    george_source.about(Uri::from("http://en.wikipedia.org/wiki/George_washington"));
    let george_citation = SourceCitation::builder("\"George Washington.\" Wikipedia, The Free Encyclopedia. Wikimedia Foundation, Inc. 24 October 2012.").build();
    george_source.citation(george_citation);
    let george_source = george_source.build();

    let mut martha_source = SourceDescription::builder();
    martha_source.id("FFF-FFFF");
    martha_source.about(Uri::from("http://en.wikipedia.org/wiki/Martha_washington"));
    let martha_citation = SourceCitation::builder("\"Martha Washington.\" Wikipedia, The Free Encyclopedia. Wikimedia Foundation, Inc. 24 October 2012.").build();
    martha_source.citation(martha_citation);
    let martha_source = martha_source.build();

    george.source(&george_source).unwrap();

    martha.source(&martha_source).unwrap();

    relationship.source(&martha_source).unwrap();

    return vec![george_source, martha_source];
}

#[test]
fn deserialize_and_serialize() {
    common::assert_matching_json(test_struct(), "spec");
}

use gedcomx::{
    Date, Fact, FactQualifier, FactType, Gedcomx, Person, PlaceReference, Qualifier, Relationship,
    RelationshipType,
};

mod common;

#[test]
fn test_census_and_residence_like_facts() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::Census)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Emigration)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Immigration)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::LandTransaction)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MoveTo)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MoveFrom)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Residence)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();

    common::assert_matching_json(&gx, "census");
    common::assert_matching_xml(&gx, "census");
}

#[test]
fn test_military_service_facts() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::MilitaryAward)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MilitaryDischarge)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MilitaryDraftRegistration)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MilitaryInduction)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MilitaryService)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    common::assert_matching_json(&gx, "military");
    common::assert_matching_xml(&gx, "military");
}

#[test]
fn test_education_and_occupation_facts() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::Apprenticeship)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Education)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Occupation)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Retirement)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    common::assert_matching_json(&gx, "education");
    common::assert_matching_xml(&gx, "education");
}

#[test]
fn test_religious_or_cultural_facts() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::AdultChristening)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Baptism)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::BarMitzvah)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::BatMitzvah)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Caste)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Christening)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Circumcision)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Clan)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Confirmation)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Excommunication)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::FirstCommunion)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Nationality)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Ordination)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Religion)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Yahrzeit)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    common::assert_matching_json(&gx, "religious");
    common::assert_matching_xml(&gx, "religious");
}

#[test]
fn test_fact_qualifiers() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::Christening)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .qualifier(Qualifier::new(FactQualifier::Religion, Some("Catholic")))
                .build(),
        )
        .fact(
            Fact::builder(FactType::Census)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .qualifier(Qualifier::new(FactQualifier::Age, Some("44")))
                .build(),
        )
        .fact(
            Fact::builder(FactType::Death)
                .date(Date::new(Some("..."), None))
                .place(PlaceReference::builder().original("...").build())
                .qualifier(Qualifier::new(FactQualifier::Cause, Some("Heart failure")))
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    common::assert_matching_json(&gx, "fact_qualifiers");
    common::assert_matching_xml(&gx, "fact_qualifiers");
}

#[test]
fn test_custom_fact() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::Custom("data:,Eagle%20Scout".into()))
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    common::assert_matching_json(&gx, "custom_facts");
    common::assert_matching_xml(&gx, "custom_facts");
}

#[test]
fn test_relationship_facts() {
    let person1 = Person::builder().id("p1").build();
    let person2 = Person::builder().id("p2").build();

    let couple = Relationship::builder(&person1, &person2)
        .unwrap()
        .relationship_type(RelationshipType::Couple)
        .fact(
            Fact::builder(FactType::CivilUnion)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::DomesticPartnership)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::Divorce)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::Marriage)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::MarriageBanns)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::MarriageContract)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::MarriageLicense)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .build();

    let parent_child = Relationship::builder(&person1, &person2)
        .unwrap()
        .relationship_type(RelationshipType::ParentChild)
        .fact(
            Fact::builder(FactType::AdoptiveParent)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::BiologicalParent)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::FosterParent)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::GuardianParent)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .fact(
            Fact::builder(FactType::StepParent)
                .place(PlaceReference::builder().original("...").build())
                .date(Date::new(Some("..."), None))
                .build(),
        )
        .build();

    let gx = Gedcomx::builder()
        .relationship(couple)
        .relationship(parent_child)
        .build();

    common::assert_matching_json(&gx, "relationships");
    common::assert_matching_xml(&gx, "relationships");
}

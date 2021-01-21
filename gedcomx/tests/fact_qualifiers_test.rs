use gedcomx::{
    Date, Fact, FactQualifier, FactType, Gedcomx, Person, PlaceReference, Qualifier, Relationship,
    RelationshipType,
};

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn test_census_and_residence_like_facts() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::Census)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Emigration)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Immigration)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::LandTransaction)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MoveTo)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MoveFrom)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Residence)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    let json = serde_json::to_string(&gx).unwrap();
    let new_gx: Gedcomx = serde_json::from_str(&json).unwrap();
    assert_eq!(gx, new_gx);
}

#[test]
fn test_military_service_facts() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::MilitaryAward)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MilitaryDischarge)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MilitaryDraftRegistration)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MilitaryInduction)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::MilitaryService)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    let json = serde_json::to_string(&gx).unwrap();
    let new_gx: Gedcomx = serde_json::from_str(&json).unwrap();
    assert_eq!(gx, new_gx);
}

#[test]
fn test_education_and_occupation_facts() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::Apprenticeship)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Education)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Occupation)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Retirement)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    let json = serde_json::to_string(&gx).unwrap();
    let new_gx: Gedcomx = serde_json::from_str(&json).unwrap();
    assert_eq!(gx, new_gx);
}

#[test]
fn test_religious_or_cultural_facts() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::AdultChristening)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Baptism)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::BarMitzvah)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::BatMitzvah)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Caste)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Christening)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Circumcision)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Clan)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Confirmation)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Excommunication)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::FirstCommunion)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Nationality)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Ordination)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Religion)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .fact(
            Fact::builder(FactType::Yahrzeit)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    let json = serde_json::to_string(&gx).unwrap();
    let new_gx: Gedcomx = serde_json::from_str(&json).unwrap();
    assert_eq!(gx, new_gx);
}

#[test]
fn test_fact_qualifiers() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::Christening)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .qualifier(Qualifier::new(FactQualifier::Religion, Some("Catholic")))
                .build(),
        )
        .fact(
            Fact::builder(FactType::Census)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .qualifier(Qualifier::new(FactQualifier::Age, Some("44")))
                .build(),
        )
        .fact(
            Fact::builder(FactType::Death)
                .date(Date::builder().original("...").build())
                .place(PlaceReference::builder().original("...").build())
                .qualifier(Qualifier::new(FactQualifier::Cause, Some("Heart failure")))
                .build(),
        )
        .build();

    let gx = Gedcomx::builder().person(person).build();
    let json_value = serde_json::to_value(&gx).unwrap();

    let expected_json = std::fs::read_to_string("../data/fact_qualifiers.json").unwrap();
    let expected_value: serde_json::Value = serde_json::from_str(&expected_json).unwrap();

    assert_eq!(json_value, expected_value);
    assert_eq!(gx, serde_json::from_str::<Gedcomx>(&expected_json).unwrap())
}

#[test]
fn test_custom_fact() {
    let person = Person::builder()
        .fact(
            Fact::builder(FactType::Custom("Eagle Scout".into()))
                .place(PlaceReference::builder().original("...").build())
                .date(Date::builder().original("...").build())
                .build(),
        )
        .build();
    let gx = Gedcomx::builder().person(person).build();
    let json = serde_json::to_string(&gx).unwrap();
    let new_gx: Gedcomx = serde_json::from_str(&json).unwrap();
    assert_eq!(gx, new_gx);
}

#[test]
fn test_relationship_facts() {
    let person1 = Person::builder().id("p1").build();
    let person2 = Person::builder().id("p2").build();

    let couple = Relationship::builder(&person1, &person2)
        .unwrap()
        .relationship_type(RelationshipType::Couple)
        .fact(Fact::builder(FactType::CivilUnion).build())
        .fact(Fact::builder(FactType::DomesticPartnership).build())
        .fact(Fact::builder(FactType::Divorce).build())
        .fact(Fact::builder(FactType::Marriage).build())
        .fact(Fact::builder(FactType::MarriageBanns).build())
        .fact(Fact::builder(FactType::MarriageContract).build())
        .fact(Fact::builder(FactType::MarriageLicense).build())
        .build();

    let parent_child = Relationship::builder(&person1, &person2)
        .unwrap()
        .relationship_type(RelationshipType::ParentChild)
        .fact(Fact::builder(FactType::AdoptiveParent).build())
        .fact(Fact::builder(FactType::BiologicalParent).build())
        .fact(Fact::builder(FactType::FosterParent).build())
        .fact(Fact::builder(FactType::GuardianParent).build())
        .fact(Fact::builder(FactType::StepParent).build())
        .build();

    let gx = Gedcomx::builder()
        .relationship(couple)
        .relationship(parent_child)
        .build();
    let json = serde_json::to_string(&gx).unwrap();
    let new_gx: Gedcomx = serde_json::from_str(&json).unwrap();
    assert_eq!(gx, new_gx);
}

use gedcomx::{
    Agent, Attribution, Date, Document, Fact, FactType, Gedcomx, GenderType, Person,
    PlaceReference, Relationship, RelationshipType, ResourceType, SourceCitation,
    SourceDescription, Timestamp,
};

mod common;

fn test_struct() -> Gedcomx {
    let contributor = Agent::builder()
        .id("A-1")
        .name("Jane Doe")
        .email("mailto:example@example.org")
        .build();

    let repository = Agent::builder()
        .id("A-2")
        .name("General Registry Office, Southport")
        .build();

    let attribution = Attribution::builder()
        .contributor(&contributor)
        .unwrap()
        .modified("2014-03-07T07:00:00".parse::<Timestamp>().unwrap())
        .change_message("change message example")
        .build();

    let source_description = SourceDescription::builder(SourceCitation::new(
        "England, birth certificate for Emma Bocock, born 23 July 1843; citing 1843 Birth in \
         District and Sub-district of Ecclesall-Bierlow in the County of York, 303; General \
         Registry Office, Southport.",
        None,
    ))
    .id("S-1")
    .title("Birth Certificate of Emma Bocock, 23 July 1843, General Registry Office")
    .resource_type(ResourceType::PhysicalArtifact)
    .created("1843-07-27T07:00:00".parse().unwrap())
    .repository(&repository)
    .unwrap()
    .build();

    let birth = Fact::builder(FactType::Birth)
        .date(Date::new(Some("23 June 1843"), None))
        .place(
            PlaceReference::builder()
                .original(
                    "Broadfield Bar, Abbeydale Road, Ecclesall-Bierlow, York, England, United \
                     Kingdom",
                )
                .build(),
        )
        .build();

    let emma = Person::builder()
        .id("P-1")
        .extracted(true)
        .source(&source_description)
        .unwrap()
        .name("Emma Bocock")
        .gender(GenderType::Female)
        .fact(birth)
        .build();

    let father = Person::builder()
        .id("P-2")
        .extracted(true)
        .source(&source_description)
        .unwrap()
        .name("William Bocock")
        .fact(
            Fact::builder(FactType::Occupation)
                .value("Toll Collector")
                .build(),
        )
        .build();

    let mother = Person::builder()
        .id("P-3")
        .extracted(true)
        .source(&source_description)
        .unwrap()
        .name("Sarah Bocock formerly Brough")
        .build();

    let father_relationship = Relationship::builder(&father, &emma)
        .unwrap()
        .relationship_type(RelationshipType::ParentChild)
        .build();

    let mother_relationship = Relationship::builder(&mother, &emma)
        .unwrap()
        .relationship_type(RelationshipType::ParentChild)
        .build();

    let analysis = Document::builder("...Jane Doe's analysis document...")
        .id("D-1")
        .build();

    let emma_conclusion = Person::builder()
        .id("C-1")
        .evidence(&emma)
        .unwrap()
        .analysis(&analysis)
        .unwrap()
        .build();

    Gedcomx::builder()
        .agent(contributor)
        .agent(repository)
        .attribution(attribution)
        .source_description(source_description)
        .person(emma)
        .person(father)
        .person(mother)
        .relationship(father_relationship)
        .relationship(mother_relationship)
        .document(analysis)
        .person(emma_conclusion)
        .build()
}

#[test]
fn test_deserialize_serialize() {
    common::assert_matching_json(&test_struct(), "birth");
    common::assert_matching_xml(&test_struct(), "birth");
}

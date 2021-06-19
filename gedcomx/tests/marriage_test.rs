use gedcomx::{
    Address, Agent, Attribution, Date, Document, DocumentType, Event, EventRole, EventRoleType,
    EventType, Fact, FactType, Gedcomx, GenderType, Person, PlaceReference, Relationship,
    RelationshipType, ResourceType, SourceCitation, SourceDescription, SourceReference, Timestamp,
};

mod common;

#[test]
fn test_example() {
    //Jane Doe, the researcher.
    let jane_doe = Agent::builder()
        .id("A-1")
        .name("Jane Doe")
        .email("mailto:example@example.org")
        .build();

    //Lin Yee Chung Cemetery
    let fhl = Agent::builder()
        .id("A-2")
        .name("Family History Library")
        .address(
            Address::builder()
                .city("Salt Lake City")
                .state_or_province("Utah")
                .build(),
        )
        .build();

    //The attribution for this research.
    let research_attribution = Attribution::builder()
        .contributor(&jane_doe)
        .unwrap()
        .modified("2014-04-25T06:00:00".parse::<Timestamp>().unwrap())
        .build();

    //The parish register.
    let record_description = SourceDescription::builder(SourceCitation::new(
        "Joseph Houghton Spencer, transcriber, Church of England, Parish Church of Wilton \
         (Somerset). A copy of the registers of the baptisms, marriages, and burials at the \
         church of St. George in the parish of Wilton : adjoining Taunton, in the county of \
         Somerset from A.D. 1558 to A.D. 1837; Marriage entry for Samuel Ham and Elizabeth \
         Spiller (3 November 1828), (Taunton: Barnicott, 1890), p. 224, No. 86.",
        None,
    ))
    .id("S-1")
    .title(
        "Marriage entry for Samuel Ham and Elizabeth Spiller, Parish Register, Wilton, Somerset, \
         England",
    )
    .description(
        "Marriage entry for Samuel Ham and Elizabeth in a copy of the registers of the baptisms, \
         marriages, and burials at the church of St. George in the parish of Wilton : adjoining \
         Taunton, in the county of Somerset from A.D. 1558 to A.D. 1837.",
    )
    .resource_type(ResourceType::PhysicalArtifact)
    .repository(&fhl)
    .unwrap()
    .build();

    //The transcription of the grave stone.
    let transcription = Document::builder(
        "Samuel Ham of the parish of Honiton and Elizabeth Spiller\nwere married this 3rd day of \
         November 1828 by David Smith\nStone, Pl Curate,\nIn the Presence of\nJno Pain.\nR.G. \
         Halls.  Peggy Hammet.\nNo. 86.",
    )
    .id("D-1")
    .lang("en")
    .document_type(DocumentType::Transcription)
    .source(&record_description)
    .unwrap()
    .build();

    //The transcription described as a source.
    let transcription_description = SourceDescription::builder(SourceCitation::new(
        "Joseph Houghton Spencer, transcriber, Church of England, Parish Church of Wilton \
         (Somerset). A copy of the registers of the baptisms, marriages, and burials at the \
         church of St. George in the parish of Wilton : adjoining Taunton, in the county of \
         Somerset from A.D. 1558 to A.D. 1837; Marriage entry for Samuel Ham and Elizabeth \
         Spiller (3 November 1828), (Taunton: Barnicott, 1890), p. 224, No. 86.",
        None,
    ))
    .id("S-2")
    .about(transcription.id.clone().unwrap().into())
    .title(
        "Transcription of marriage entry for Samuel Ham and Elizabeth Spiller, Parish Register, \
         Wilton, Somerset, England",
    )
    .description(
        "Transcription of marriage entry for Samuel Ham and Elizabeth in a copy of the registers \
         of the baptisms, marriages, and burials at the church of St. George in the parish of \
         Wilton : adjoining Taunton, in the county of Somerset from A.D. 1558 to A.D. 1837.",
    )
    .resource_type(ResourceType::DigitalArtifact)
    .source(
        SourceReference::builder(&record_description)
            .unwrap()
            .build(),
    )
    .build();

    //the marriage fact.
    let marriage = Fact::builder(FactType::Marriage)
        .date(Date::new(
            Some("3 November 1828"),
            Some("+1828-11-03".parse().unwrap()),
        ))
        .place(
            PlaceReference::builder()
                .original("Wilton St George, Wilton, Somerset, England")
                .build(),
        )
        .build();

    //the spouse1's residence.
    let sams_residence = Fact::builder(FactType::Residence)
        .date(Date::new(
            Some("3 November 1828"),
            Some("+1828-11-03".parse().unwrap()),
        ))
        .place(
            PlaceReference::builder()
                .original("parish of Honiton, Devon, England")
                .build(),
        )
        .build();

    //the spouse1's residence.
    let lizs_residence = Fact::builder(FactType::Residence)
        .date(Date::new(
            Some("3 November 1828"),
            Some("+1828-11-03".parse().unwrap()),
        ))
        .place(
            PlaceReference::builder()
                .original("parish of Wilton, Somerset, England")
                .build(),
        )
        .build();

    //the spouse1
    let sam = Person::builder()
        .id("P-1")
        .extracted(true)
        .source(&transcription_description)
        .unwrap()
        .name("Samuel Ham")
        .gender(GenderType::Male)
        .fact(sams_residence)
        .build();

    //the spouse2.
    let liz = Person::builder()
        .id("P-2")
        .extracted(true)
        .source(&transcription_description)
        .unwrap()
        .name("Elizabeth Spiller")
        .gender(GenderType::Female)
        .fact(lizs_residence)
        .build();

    //witnesses
    let witness1 = Person::builder()
        .id("P-3")
        .extracted(true)
        .source(&transcription_description)
        .unwrap()
        .name("Jno. Pain")
        .build();
    let witness2 = Person::builder()
        .id("P-4")
        .extracted(true)
        .source(&transcription_description)
        .unwrap()
        .name("R.G. Halls")
        .build();
    let witness3 = Person::builder()
        .id("P-5")
        .extracted(true)
        .source(&transcription_description)
        .unwrap()
        .name("Peggy Hammet")
        .build();

    //officiator
    let officiator = Person::builder()
        .id("P-6")
        .extracted(true)
        .source(&transcription_description)
        .unwrap()
        .name("David Smith Stone")
        .build();

    //the relationship.
    let marriage_relationship = Relationship::builder(&sam, &liz)
        .unwrap()
        .extracted(true)
        .relationship_type(RelationshipType::Couple)
        .fact(marriage)
        .build();

    //the marriage event
    let marriage_event = Event::builder()
        .event_type(EventType::Marriage)
        .id("E-1")
        .extracted(true)
        .date(Date::new(
            Some("3 November 1828"),
            Some("+1828-11-03".parse().unwrap()),
        ))
        .place(
            PlaceReference::builder()
                .original("Wilton St George, Wilton, Somerset, England")
                .build(),
        )
        .role(
            EventRole::builder(&sam)
                .unwrap()
                .event_role_type(EventRoleType::Principal)
                .build(),
        )
        .role(
            EventRole::builder(&liz)
                .unwrap()
                .event_role_type(EventRoleType::Principal)
                .build(),
        )
        .role(
            EventRole::builder(&witness1)
                .unwrap()
                .event_role_type(EventRoleType::Witness)
                .build(),
        )
        .role(
            EventRole::builder(&witness2)
                .unwrap()
                .event_role_type(EventRoleType::Witness)
                .build(),
        )
        .role(
            EventRole::builder(&witness3)
                .unwrap()
                .event_role_type(EventRoleType::Witness)
                .build(),
        )
        .role(
            EventRole::builder(&officiator)
                .unwrap()
                .event_role_type(EventRoleType::Official)
                .build(),
        )
        .build();

    //Jane Doe's analysis.
    let analysis = Document::builder("...Jane Doe's analysis document...")
        .id("D-2")
        .build();

    //Jane Doe's conclusions about a person.
    let sam_conclusion = Person::builder()
        .id("C-1")
        .evidence(&sam)
        .unwrap()
        .analysis(&analysis)
        .unwrap()
        .build();

    let gx = Gedcomx::builder()
        .agent(jane_doe)
        .agent(fhl)
        .attribution(research_attribution)
        .source_description(record_description)
        .document(transcription)
        .source_description(transcription_description)
        .person(sam)
        .person(liz)
        .person(witness1)
        .person(witness2)
        .person(witness3)
        .person(officiator)
        .relationship(marriage_relationship)
        .event(marriage_event)
        .document(analysis)
        .person(sam_conclusion)
        .build();

    common::assert_matching_json(&gx, "marriage");
    common::assert_matching_xml(&gx, "marriage");
}

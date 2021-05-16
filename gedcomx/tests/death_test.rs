use gedcomx::{
    Address, Agent, Attribution, Date, Document, Fact, FactType, Gedcomx, Gender, GenderType, Name,
    NameForm, NameType, Person, PlaceReference, Relationship, RelationshipType, ResourceType,
    SourceCitation, SourceDescription, SourceReference, Timestamp,
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
    let cemetery = Agent::builder()
        .id("A-2")
        .name("Lin Yee Chung Cemetery")
        .address(
            Address::builder()
                .city("Honolulu")
                .state_or_province("Hawaii")
                .build(),
        )
        .build();

    //Hanyu Pinyin, the translator.
    let hanyu_pinyin = Agent::builder()
        .id("A-3")
        .name("HANYU Pinyin 王大年")
        .email("mailto:example@example.org")
        .build();

    //The attribution for this research.
    let research_attribution = Attribution::builder()
        .contributor(&jane_doe)
        .unwrap()
        .modified("2014-03-27T06:00:00".parse::<Timestamp>().unwrap())
        .build();

    //The attribution for the translation.
    let translation_attribution = Attribution::builder()
        .contributor(&hanyu_pinyin)
        .unwrap()
        .modified("2014-03-27T06:00:00".parse::<Timestamp>().unwrap())
        .build();

    //The grave stone.
    let gravestone_description = SourceDescription::builder()
        .id("S-1")
        .title("Grave Marker of WONG Aloiau, Lin Yee Chung Cemetery, Honolulu, Oahu, Hawaii")
        .citation(
            SourceCitation::builder(
                "WONG Aloiau gravestone, Lin Yee Chung Cemetery, Honolulu, Oahu, Hawaii; visited \
                 May 1975 by Jane Doe.",
            )
            .build(),
        )
        .resource_type(ResourceType::PhysicalArtifact)
        .repository(&cemetery)
        .unwrap()
        .build();

    //The image of the grave stone.
    let gravestone_image_description = SourceDescription::builder()
        .id("S-2")
        .title("Grave Marker of WONG Aloiau, Lin Yee Chung Cemetery, Honolulu, Oahu, Hawaii")
        .citation(
            SourceCitation::builder(
                "WONG Aloiau gravestone (digital photograph), Lin Yee Chung Cemetery, Honolulu, \
                 Oahu, Hawaii; visited May 1975 by Jane Doe.",
            )
            .build(),
        )
        .resource_type(ResourceType::DigitalArtifact)
        .source(
            SourceReference::builder(&gravestone_description)
                .unwrap()
                .build(),
        )
        .build();

    //The transcription of the grave stone.
    let transcription = Document::builder(
        "WONG ALOIAU\nNOV. 22, 1848 – AUG. 3, 1920\n中山  大字都  泮沙鄉\n生  於  前  清 戊申 年 \
         十一 月 廿二（日）子   時\n終  於  民國  庚申 年     七月    十二 (日)    午    時\n先考  \
         諱 羅有  字 容康 王 府 君 之 墓",
    )
    .id("D-1")
    .lang("zh")
    .source(&gravestone_image_description)
    .unwrap()
    .build();

    //The transcription described as a source.
    let transcription_description = SourceDescription::builder()
        .id("S-3")
        .about(transcription.id.clone().unwrap().into())
        .title(
            "Transcription of Grave Marker of WONG Aloiau, Lin Yee Chung Cemetery, Honolulu, \
             Oahu, Hawaii",
        )
        .citation(
            SourceCitation::builder(
                "WONG Aloiau gravestone (transcription), Lin Yee Chung Cemetery, Honolulu, Oahu, \
                 Hawaii; visited May 1975 by Jane Doe.",
            )
            .build(),
        )
        .resource_type(ResourceType::DigitalArtifact)
        .source(
            SourceReference::builder(&gravestone_image_description)
                .unwrap()
                .build(),
        )
        .build();

    //The translation of the grave stone.
    let translation = Document::builder(
        "WONG ALOIAU\nNOV. 22, 1848 – AUG. 3, 1920 [lunar dates]\n[Birthplace] [China, Guandong, \
         ]Chung Shan, See Dai Doo, Pun Sha village\n[Date of birth] Born at former Qing 1848 year \
         11th month 22nd day 23-1 hour.\n[Life] ended at Republic of China year 1920 year 7th mo. \
         12th day 11-13 hour.\nDeceased father avoid [mention of] Lo Yau also known as Young Hong \
         Wong [noble]residence ruler’s grave.",
    )
    .id("D-2")
    .source(&transcription_description)
    .unwrap()
    .build();

    //The translation described as a source.
    let translation_description = SourceDescription::builder()
        .id("S-4")
        .about(translation.id.clone().unwrap().into())
        .title(
            "Translation of Grave Marker of WONG Aloiau, Lin Yee Chung Cemetery, Honolulu, Oahu, \
             Hawaii",
        )
        .citation(
            SourceCitation::builder(
                "WONG Aloiau gravestone, Lin Yee Chung Cemetery, Honolulu, Oahu, Hawaii; visited \
                 May 1975 by Jane Doe. Translation by HANYU Pinyin 王大年.",
            )
            .build(),
        )
        .attribution(translation_attribution)
        .resource_type(ResourceType::DigitalArtifact)
        .source(
            SourceReference::builder(&transcription_description)
                .unwrap()
                .build(),
        )
        .build();

    //the birth.
    let birth = Fact::builder(FactType::Birth)
        .date(Date::new(
            Some("former Qing 1848 year 11th month 22nd day 23-1 hour"),
            Some("+1848-11-22".parse().unwrap()),
        ))
        .place(
            PlaceReference::builder()
                .original("Pun Sha Village, See Dai Doo, Chung Shan, Guangdong, China")
                .build(),
        )
        .build();

    //the death.
    let death = Fact::builder(FactType::Death)
        .date(Date::new(
            Some("Republic of China year 1920 year 7th mo. 12th day 11-13 hour"),
            Some("+1920-08-03".parse().unwrap()),
        ))
        .build();

    //the burial.
    let burial = Fact::builder(FactType::Burial)
        .place(
            PlaceReference::builder()
                .original("Lin Yee Chung Cemetery, Honolulu, Oahu, Hawaii")
                .build(),
        )
        .build();

    //the principal person
    let aloiau = Person::builder()
        .id("P-1")
        .extracted(true)
        .source(&translation_description)
        .unwrap()
        .name("WONG Aloiau")
        .gender(Gender::from(GenderType::Male))
        .fact(birth)
        .fact(death)
        .fact(burial)
        .build();

    //the father of the principal (with an aka name).
    let father = Person::builder()
        .id("P-2")
        .extracted(true)
        .source(&translation_description)
        .unwrap()
        .name("Lo Yau")
        .name(
            Name::builder()
                .name_type(NameType::AlsoKnownAs)
                .name_form(NameForm::builder().full_text("Young Hong Wong").build())
                .build(),
        )
        .build();

    //the relationship.
    let father_relationship = Relationship::builder(&father, &aloiau)
        .unwrap()
        .relationship_type(RelationshipType::ParentChild)
        .build();

    //Jane Doe's analysis.
    let analysis = Document::builder("...Jane Doe's analysis document...")
        .id("D-3")
        .build();

    //Jane Doe's conclusions about a person.
    let aloiau_conclusion = Person::builder()
        .id("C-1")
        .evidence(&aloiau)
        .unwrap()
        .analysis(&analysis)
        .unwrap()
        .build();

    let gx = Gedcomx::builder()
        .agent(jane_doe)
        .agent(hanyu_pinyin)
        .agent(cemetery)
        .attribution(research_attribution)
        .source_description(gravestone_description)
        .source_description(gravestone_image_description)
        .document(transcription)
        .source_description(transcription_description)
        .document(translation)
        .source_description(translation_description)
        .person(aloiau)
        .person(father)
        .relationship(father_relationship)
        .document(analysis)
        .person(aloiau_conclusion)
        .build();

    common::assert_matching_json(&gx, "death");
    common::assert_matching_xml(&gx, "death");
}

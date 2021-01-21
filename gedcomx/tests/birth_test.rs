use chrono::{DateTime, NaiveDateTime, ParseResult, Utc};
use gedcomx::{
    Agent, Attribution, Date, Document, Fact, FactType, Gedcomx, GenderType, Person,
    PlaceReference, Relationship, RelationshipType, ResourceType, SourceCitation,
    SourceDescription,
};

#[cfg(test)]
use pretty_assertions::assert_eq;

fn parse(s: &str) -> ParseResult<DateTime<Utc>> {
    let date_time = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?;
    Ok(DateTime::from_utc(date_time, Utc))
}

fn test_struct() -> Gedcomx {
    let contributor = Agent::builder()
        .id("A-1")
        .name("Jane Doe")
        .email("example@example.org")
        .build();

    let repository = Agent::builder()
        .id("A-2")
        .name("General Registry Office, Southport")
        .build();

    let attribution = Attribution::builder()
        .contributor(&contributor)
        .unwrap()
        .modified(parse("2014-03-07 07:00:00").unwrap())
        .change_message("change message example")
        .build();

    let source_description = SourceDescription::builder().id("S-1")
          .title("Birth Certificate of Emma Bocock, 23 July 1843, General Registry Office")
          .citation(SourceCitation::builder("England, birth certificate for Emma Bocock, born 23 July 1843; citing 1843 Birth in District and Sub-district of Ecclesall-Bierlow in the County of York, 303; General Registry Office, Southport.").build())
          .resource_type(ResourceType::PhysicalArtifact)
          .created(parse("1843-07-27 07:00:00").unwrap())
          .repository(&repository).unwrap().build();

    let birth = Fact::builder(FactType::Birth)
          .date(Date::builder().original("23 June 1843").build())
          .place(PlaceReference::builder().original("Broadfield Bar, Abbeydale Road, Ecclesall-Bierlow, York, England, United Kingdom").build()).build();

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

fn test_json() -> String {
    std::fs::read_to_string("../data/birth.json").unwrap()
}

#[test]
fn deserialize() {
    assert_eq!(
        serde_json::from_str::<Gedcomx>(&test_json()).unwrap(),
        test_struct()
    )
}

#[test]
fn serialize() {
    // Instead of comparing string, which may yield false negatives because of whitespace, etc,
    // we'll compare serde_json::Values, which is loosly typed json.
    let actual = serde_json::to_value(test_struct()).unwrap();
    let expected: serde_json::Value = serde_json::from_str(&test_json()).unwrap();
    assert_eq!(actual, expected)
}

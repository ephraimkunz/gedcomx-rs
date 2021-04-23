use gedcomx::*;

fn main() {
    env_logger::init();
    xml_deserialize();
}

fn xml_deserialize() {
    let xml = r##"<person extracted="true" id="P-2">
    <source description="#S-4"/>
    <name>
        <nameForm>
            <fullText>Lo Yau</fullText>
        </nameForm>
    </name>
    <name type="http://gedcomx.org/AlsoKnownAs">
        <nameForm>
            <fullText>Young Hong Wong</fullText>
        </nameForm>
    </name>
</person>"##;

    let expected_person = Person::builder()
        .extracted(true)
        .id("P-2")
        .source_ref(SourceReference::new("#S-4".into(), None, None, vec![]))
        .name(
            Name::builder()
                .name_form(NameForm::builder().full_text("Lo Yau").build())
                .build(),
        )
        .name(
            Name::builder()
                .name_type(NameType::AlsoKnownAs)
                .name_form(NameForm::builder().full_text("Young Hong Wong").build())
                .build(),
        )
        .build();
    let person: Person = yaserde::de::from_str(xml).unwrap();
    println!("{:?}", person);
    // assert_eq!(person, expected_person)
}

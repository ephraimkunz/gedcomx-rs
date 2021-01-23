use gedcomx::{
    Gedcomx, Name, NameForm, NamePart, NamePartQualifier, NamePartType, Person, Qualifier,
};

mod common;

#[test]
fn test_basic_western_name() {
    let name_form = NameForm::builder()
        .full_text("John Fitzgerald Kennedy")
        .lang("en")
        .part(
            NamePart::builder("John")
                .part_type(NamePartType::Given)
                .build(),
        )
        .part(
            NamePart::builder("Fitzgerald")
                .part_type(NamePartType::Given)
                .build(),
        )
        .part(
            NamePart::builder("Kennedy")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .build();

    let name = Name::builder().name_form(name_form).build();

    let gx = Gedcomx::builder()
        .person(Person::builder().name(name).build())
        .build();

    common::assert_matching_json(gx, "names1");
}

#[test]
fn test_multiple_japanese_forms() {
    let kanji = NameForm::builder()
        .full_text("山田太郎")
        .lang("ja-Hani")
        .part(
            NamePart::builder("山田")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .part(
            NamePart::builder("太郎")
                .part_type(NamePartType::Given)
                .build(),
        )
        .build();

    let katakana = NameForm::builder()
        .full_text("ヤマダタロー")
        .lang("ja-Kana")
        .part(
            NamePart::builder("ヤマダ")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .part(
            NamePart::builder("タロー")
                .part_type(NamePartType::Given)
                .build(),
        )
        .build();

    let romanized = NameForm::builder()
        .full_text("Yamada Tarō")
        .lang("ja-Latn")
        .part(
            NamePart::builder("Tarō")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .part(
            NamePart::builder("Yamada")
                .part_type(NamePartType::Given)
                .build(),
        )
        .build();

    let name = Name::builder()
        .name_form(kanji)
        .name_form(katakana)
        .name_form(romanized)
        .build();
    let gx = Gedcomx::builder()
        .person(Person::builder().name(name).build())
        .build();

    common::assert_matching_json(gx, "names2");
}

#[test]
fn test_multiple_name_parts_one_part_per_type() {
    let name_form = NameForm::builder()
        .full_text("José Eduardo Santos Tavares Melo Silva")
        .lang("pt-BR")
        .part(
            NamePart::builder("José Eduardo")
                .part_type(NamePartType::Given)
                .build(),
        )
        .part(
            NamePart::builder("Santos Tavares Melo Silva")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .build();
    let name = Name::builder().name_form(name_form).build();
    let gx = Gedcomx::builder()
        .person(Person::builder().name(name).build())
        .build();

    common::assert_matching_json(gx, "names3");
}

#[test]
fn test_multiple_name_parts_multiple_parts_per_type() {
    let name_form = NameForm::builder()
        .full_text("José Eduardo Santos Tavares Melo Silva")
        .lang("pt-BR")
        .part(
            NamePart::builder("José")
                .part_type(NamePartType::Given)
                .build(),
        )
        .part(
            NamePart::builder("Eduardo")
                .part_type(NamePartType::Given)
                .build(),
        )
        .part(
            NamePart::builder("Santos")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .part(
            NamePart::builder("Tavares")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .part(
            NamePart::builder("Melo")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .part(
            NamePart::builder("Silva")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .build();
    let name = Name::builder().name_form(name_form).build();

    let gx = Gedcomx::builder()
        .person(Person::builder().name(name).build())
        .build();
    common::assert_matching_json(gx, "names4")
}

#[test]
fn test_patronymic() {
    let name_form = NameForm::builder()
        .full_text("Björk Guðmundsdóttir")
        .lang("is")
        .part(
            NamePart::builder("Björk")
                .part_type(NamePartType::Given)
                .build(),
        )
        .part(
            NamePart::builder("Guðmundsdóttir")
                .qualifier(Qualifier::new::<NamePartQualifier, String>(
                    NamePartQualifier::Patronymic,
                    None,
                ))
                .build(),
        )
        .build();
    let name = Name::builder().name_form(name_form).build();

    let gx = Gedcomx::builder()
        .person(Person::builder().name(name).build())
        .build();
    common::assert_matching_json(gx, "names5")
}

#[test]
fn test_get_part() {
    let name_form = NameForm::builder()
        .full_text("John Fitzgerald Kennedy")
        .lang("en")
        .part(
            NamePart::builder("John")
                .part_type(NamePartType::Given)
                .build(),
        )
        .part(
            NamePart::builder("Fitzgerald")
                .part_type(NamePartType::Given)
                .build(),
        )
        .part(
            NamePart::builder("Kennedy")
                .part_type(NamePartType::Surname)
                .build(),
        )
        .build();

    let name = Name::builder().name_form(name_form).build();

    assert_eq!("John", name.part_for_type(&NamePartType::Given).unwrap());
    assert_eq!(
        "Kennedy",
        name.part_for_type(&NamePartType::Surname).unwrap(),
    );

    let name_no_forms = Name::builder().build();
    assert_eq!(None, name_no_forms.part_for_type(&NamePartType::Given));
    assert_eq!(None, name_no_forms.part_for_type(&NamePartType::Surname));

    let name_form_no_parts = NameForm::builder()
        .full_text("John Fitzgerald Kennedy")
        .lang("en")
        .build();
    let name_no_parts = Name::builder().name_form(name_form_no_parts).build();
    assert_eq!(None, name_no_parts.part_for_type(&NamePartType::Given));
    assert_eq!(None, name_no_parts.part_for_type(&NamePartType::Surname));
}

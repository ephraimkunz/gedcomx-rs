use gedcomx::{Gedcomx, Name, NameForm, NamePart, NamePartType, Person};

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

// #[test]
// fn test_multiple_name_parts_one_part_per_type()  {
//   NameForm nameForm = new NameForm("José Eduardo Santos Tavares Melo Silva")
//     .lang("pt-BR")
//     .part(NamePartType.Given, "José Eduardo")
//     .part(NamePartType.Surname, "Santos Tavares Melo Silva");
//   Name name = new Name().nameForm(nameForm);

//   Gedcomx gx = new Gedcomx().person(new Person().name(name));
//   SerializationUtil.processThroughXml(gx);
//   SerializationUtil.processThroughJson(gx);
// }

// #[test]
// fn test_multiple_name_parts_multiple_parts_per_type()  {
//   NameForm nameForm = new NameForm("José Eduardo Santos Tavares Melo Silva")
//     .lang("pt-BR")
//     .part(NamePartType.Given, "José")
//     .part(NamePartType.Given, "Eduardo")
//     .part(NamePartType.Surname, "Santos")
//     .part(NamePartType.Surname, "Tavares")
//     .part(NamePartType.Surname, "Melo")
//     .part(NamePartType.Surname, "Silva");
//   Name name = new Name().nameForm(nameForm);

//   Gedcomx gx = new Gedcomx().person(new Person().name(name));
//   SerializationUtil.processThroughXml(gx);
//   SerializationUtil.processThroughJson(gx);
// }

// #[test]
// fn test_patronymic()  {
//   NameForm nameForm = new NameForm("Björk Guðmundsdóttir")
//     .lang("is")
//     .part(NamePartType.Given, "Björk")
//     .part(new NamePart().value("Guðmundsdóttir").qualifier(new Qualifier(NamePartQualifierType.Patronymic)));
//   Name name = new Name().nameForm(nameForm);

//   Gedcomx gx = new Gedcomx().person(new Person().name(name));
//   SerializationUtil.processThroughXml(gx);
//   SerializationUtil.processThroughJson(gx);
// }

// #[test]
// fn test_get_part()  {
//   NameForm nameForm = new NameForm("John Fitzgerald Kennedy")
//     .lang("en")
//     .part(NamePartType.Given, "John")
//     .part(NamePartType.Given, "Fitzgerald")
//     .part(NamePartType.Surname, "Kennedy");
//   Name name = new Name().nameForm(nameForm);
//   assertEquals("John", name.getPart(NamePartType.Given));
//   assertEquals("Kennedy", name.getPart(NamePartType.Surname));

//   Name nameNoForms = new Name();
//   assertNull(nameNoForms.getPart(NamePartType.Given));
//   assertNull(nameNoForms.getPart(NamePartType.Surname));

//   Name nameNullForm = new Name().nameForm(null);
//   assertNull(nameNullForm.getPart(NamePartType.Given));
//   assertNull(nameNullForm.getPart(NamePartType.Surname));

//   NameForm nameFormNoParts = new NameForm("John Fitzgerald Kennedy")
//     .lang("en");
//   Name nameNoParts = new Name().nameForm(nameFormNoParts);
//   assertNull(nameNoParts.getPart(NamePartType.Given));
//   assertNull(nameNoParts.getPart(NamePartType.Surname));

//   NameForm nameFormNullParts = new NameForm("John Fitzgerald Kennedy")
//     .lang("en")
//     .part(NamePartType.Given, null)
//     .part(NamePartType.Surname, null);
//   Name nameNullParts = new Name().nameForm(nameFormNullParts);
//   assertNull(nameNullParts.getPart(NamePartType.Given));
//   assertNull(nameNullParts.getPart(NamePartType.Surname));
// }

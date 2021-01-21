// #[test]
// fn testBasicWesternName()  {
//   NameForm nameForm = new NameForm("John Fitzgerald Kennedy")
//     .lang("en")
//     .part(NamePartType.Given, "John")
//     .part(NamePartType.Given, "Fitzgerald")
//     .part(NamePartType.Surname, "Kennedy");
//   Name name = new Name().nameForm(nameForm);

//   Gedcomx gx = new Gedcomx().person(new Person().name(name));
//   SerializationUtil.processThroughXml(gx);
//   SerializationUtil.processThroughJson(gx);
// }

// #[test]
// fn testMultipleJapaneseForms()  {
//   NameForm kanji = new NameForm("山田太郎")
//     .lang("ja-Hani")
//     .part(NamePartType.Surname, "山田")
//     .part(NamePartType.Given, "太郎");
//   NameForm katakana = new NameForm("ヤマダタロー")
//     .lang("ja-Kana")
//     .part(NamePartType.Surname, "ヤマダ")
//     .part(NamePartType.Given, "タロー");
//   NameForm romanized = new NameForm("Yamada Tarō")
//     .lang("ja-Latn")
//     .part(NamePartType.Surname, "Tarō")
//     .part(NamePartType.Given, "Yamada");
//   Name name = new Name().nameForm(kanji).nameForm(katakana).nameForm(romanized);

//   Gedcomx gx = new Gedcomx().person(new Person().name(name));
//   SerializationUtil.processThroughXml(gx);
//   SerializationUtil.processThroughJson(gx);
// }

// #[test]
// fn testMultipleNamePartsOnePartPerType()  {
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
// fn testMultipleNamePartsMultiplePartsPerType()  {
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
// fn testPatronymic()  {
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
// fn testGetPart()  {
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

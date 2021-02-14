// use gedcomx::Address;

fn main() {
    // let t = Address::builder()
    //     .city("East Palo Alto")
    //     .state_or_province("California")
    //     .build();
    // println!(
    //     "{}",
    //     yaserde::ser::to_string_with_config(
    //         &t,
    //         &Config {
    //             perform_indent: true,
    //             write_document_declaration: false,
    //             indent_string: Some("hi".to_string())
    //         }
    //     )
    //     .unwrap()
    // );

    // let test = EventRoleType::Custom(Uri("hi".to_string()));

    // println!("{}", yaserde::ser::to_string(&test).unwrap());
}

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// pub struct Uri(String);

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// #[non_exhaustive]
// pub enum EventRoleType {
//     /// The person is the principal person of the event.
//     ///
//     /// For example, the principal of a birth event is the person that was born.
//     Principal,

//     /// A participant in the event.
//     Participant,

//     /// A person officiating the event.
//     Official,

//     /// A witness of the event.
//     Witness,
//     Custom(Uri),
// }

// impl yaserde::YaSerialize for EventRoleType {
//     fn serialize<W: std::io::Write>(
//         &self,
//         writer: &mut yaserde::ser::Serializer<W>,
//     ) -> Result<(), String> {
//         let _ret = writer.write(xml::writer::XmlEvent::characters(&self.to_string()));
//         Ok(())
//     }

//     fn serialize_attributes(
//         &self,
//         attributes: Vec<xml::attribute::OwnedAttribute>,
//         namespace: xml::namespace::Namespace,
//     ) -> Result<
//         (
//             Vec<xml::attribute::OwnedAttribute>,
//             xml::namespace::Namespace,
//         ),
//         String,
//     > {
//         Ok((attributes, namespace))
//     }
// }

// impl yaserde::YaDeserialize for EventRoleType {
//     fn deserialize<R: std::io::Read>(
//         reader: &mut yaserde::de::Deserializer<R>,
//     ) -> Result<Self, String> {
//         if let xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()?.to_owned() {
//             let expected_name = "Alpha".to_owned();
//             if name.local_name != expected_name {
//                 return Err(format!(
//                     "Wrong StartElement name: {}, expected: {}",
//                     name, expected_name
//                 ));
//             }
//             let _next = reader.next_event();
//         } else {
//             return Err("StartElement missing".to_string());
//         }

//         if let xml::reader::XmlEvent::Characters(text) = reader.peek()?.to_owned() {
//             Ok(Self::Principal)
//         } else {
//             Err("Characters missing".to_string())
//         }
//     }
// }

// impl Display for EventRoleType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         write!(f, "hi")
//     }
// }

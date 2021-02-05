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

    let test = RR {
        reference: "hi".to_string(), //("test".to_string()),
    };

    println!("{}", yaserde::ser::to_string(&test).unwrap());
}

// use yaserde_derive::{YaDeserialize, YaSerialize};
// use serde::{Deserialize, Serialize};
// use std::io::Write;
// use yaserde::YaSerialize;

#[derive(yaserde_derive::YaSerialize, yaserde_derive::YaDeserialize)]
pub struct RR {
    reference: String,
}
// #[derive(Serialize, Deserialize)]
// #[non_exhaustive]
// pub struct Uri(String);

// impl YaSerialize for Uri {
//     fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
//         let _ret = writer.write(xml::writer::XmlEvent::characters(&self.0));
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

// impl yaserde::YaDeserialize for Uri {
//     fn deserialize<R: std::io::Read>(
//         reader: &mut yaserde::de::Deserializer<R>,
//     ) -> Result<Self, String> {
//         if let xml::reader::XmlEvent::Characters(text) = reader.peek()?.to_owned() {
//             Ok(Self(text))
//         } else {
//             Err("Characters missing".to_string())
//         }
//     }
// }

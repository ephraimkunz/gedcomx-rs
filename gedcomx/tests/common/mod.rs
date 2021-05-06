#![allow(dead_code)]

use assert_json_diff::assert_json_eq;
use gedcomx::Gedcomx;
#[cfg(test)]
use pretty_assertions::assert_eq;

fn read_json(filename: &str) -> String {
    std::fs::read_to_string(format!("../data/{}.json", filename)).unwrap()
}

fn read_xml(filename: &str, whitespace: bool) -> String {
    if whitespace {
        std::fs::read_to_string(format!("../data/{}.xml", filename)).unwrap()
    } else {
        std::fs::read_to_string(format!("../data/{}_nowhitespace.xml", filename)).unwrap()
    }
}

pub fn assert_roundtrip_xml(filename: &str) {
    let xml = read_xml(filename, true);
    let gx = yaserde::de::from_str::<Gedcomx>(&xml).unwrap();
    let new_xml = yaserde::ser::to_string(&gx).unwrap();

    assert_eq!(new_xml, xml);
}

pub fn assert_roundtrip_json(filename: &str) {
    let json = read_json(filename);
    let gx = serde_json::from_str::<Gedcomx>(&json).unwrap();
    let new_json = serde_json::to_string(&gx).unwrap();
    assert_json_eq!(
        serde_json::from_str::<serde_json::Value>(&json).unwrap(),
        serde_json::from_str::<serde_json::Value>(&new_json).unwrap()
    );
}

pub fn assert_matching_json(gx: &Gedcomx, filename: &str) {
    // Instead of comparing string, which may yield false negatives because of
    // whitespace, etc, we'll compare serde_json::Values, which is loosly typed
    // json.

    let json_value = serde_json::to_value(gx).unwrap();

    let expected_json = read_json(filename);
    let expected_value: serde_json::Value = serde_json::from_str(&expected_json).unwrap();

    assert_eq!(json_value, expected_value);
    assert_eq!(
        &serde_json::from_str::<Gedcomx>(&expected_json).unwrap(),
        gx
    )
}

pub fn assert_matching_xml(gx: &Gedcomx, filename: &str) {
    // Start a logger. To see logs from yaserde, run a test like this:
    // RUST_LOG=debug cargo test --package gedcomx --test marriage_test
    let _ = env_logger::builder().is_test(true).try_init();

    let file_string = read_xml(filename, true);
    let file_string_nowhitespace = read_xml(filename, false);

    let file_des: Gedcomx = yaserde::de::from_str(&file_string).unwrap();
    assert_eq!(&file_des, gx);

    let object_ser = yaserde::ser::to_string_with_config(
        gx,
        &yaserde::ser::Config {
            write_document_declaration: false,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(object_ser, file_string_nowhitespace);
}

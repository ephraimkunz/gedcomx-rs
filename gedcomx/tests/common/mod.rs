use gedcomx::Gedcomx;
#[cfg(test)]
use pretty_assertions::assert_eq;

pub fn assert_matching_json(gx: &Gedcomx, filename: &str) {
    // Instead of comparing string, which may yield false negatives because of
    // whitespace, etc, we'll compare serde_json::Values, which is loosly typed
    // json.

    let json_value = serde_json::to_value(gx).unwrap();

    let expected_json = std::fs::read_to_string(format!("../data/{}.json", filename)).unwrap();
    let expected_value: serde_json::Value = serde_json::from_str(&expected_json).unwrap();

    assert_eq!(json_value, expected_value);
    assert_eq!(
        &serde_json::from_str::<Gedcomx>(&expected_json).unwrap(),
        gx
    )
}

#[allow(dead_code)]
pub fn assert_matching_xml(gx: &Gedcomx, filename: &str) {
    // let file_string = std::fs::read_to_string(format!("../data/{}.xml", filename)).unwrap();
    let file_string_nowhitespace =
        std::fs::read_to_string(format!("../data/{}_nowhitespace.xml", filename)).unwrap();

    // let file_des: Gedcomx = yaserde::de::from_str(&file_string).unwrap();
    // assert_eq!(&file_des, gx);

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

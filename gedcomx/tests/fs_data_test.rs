mod common;

#[test]
fn test_current_person() {
    common::assert_roundtrip_json("current_person_fs");
}

#[test]
fn test_parents() {
    common::assert_roundtrip_xml("parents_fs");
}

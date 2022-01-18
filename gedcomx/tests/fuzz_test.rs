#[test]
fn place_description_crash() {
    let s = "<T>R<place>>/><longitude>><Pt";
    let _ = gedcomx::Gedcomx::from_xml_str(s);
}

#[test]
fn relationship_crash() {
    let s = "<HR><g/><relationship>me/><extracted>>R><R";
    let _ = gedcomx::Gedcomx::from_xml_str(s);
}

#[test]
fn group_crash() {
    let s = "<group><extracted>/<R/";
    let a = gedcomx::Gedcomx::from_xml_str(s);
    println!("{:?}", a);
}

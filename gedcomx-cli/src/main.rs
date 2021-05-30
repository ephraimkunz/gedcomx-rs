fn main() {
    let big_file = std::fs::read_to_string("data/parents_fs.xml").unwrap();
    let gx = gedcomx::Gedcomx::from_xml_str(&big_file).unwrap();
    println!("{}", gx.to_xml_string_pretty().unwrap())
}

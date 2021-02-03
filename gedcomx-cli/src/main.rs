use gedcomx::Gedcomx;

fn main() {
    for dir_entry in std::fs::read_dir("data").unwrap() {
        let path = dir_entry.unwrap().path();
        let json = std::fs::read_to_string(path.clone()).unwrap();
        match serde_json::from_str::<Gedcomx>(&json) {
            Ok(g) => println!("Success: {:?}\n{:?}\n", path, g),
            Err(e) => println!("Failure: {:?} -> {:?}\n", path, e),
        };
    }
}

fn main() {
    let gx = gedcomx::Gedcomx {
        id: Some("identifier".to_string()),
        attribution: None,
        lang: None,
        persons: vec![],
        relationships: vec![],
        source_descriptions: vec![],
        agents: vec![],
        events: vec![],
        documents: vec![],
        places: vec![],
        groups: vec![],
        description: None,
    };

    println!("{}", serde_json::to_string(&gx));
}

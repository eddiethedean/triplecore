use triplecore::{Graph, Iri, Literal, NamespaceRegistry};

#[test]
fn graph_core_example() {
    let mut graph = Graph::new();
    let subject = Iri::new("https://example.com/person/1").unwrap();
    let name = Iri::new("https://schema.org/name").unwrap();
    graph
        .add_iri_literal(subject, name, Literal::string("Ada Lovelace"))
        .unwrap();

    assert_eq!(graph.len(), 1);

    let json = graph.to_json_pretty().unwrap();
    let restored = Graph::from_json(&json).unwrap();
    assert_eq!(graph, restored);
}

#[test]
fn namespace_registry_common_prefixes() {
    let registry = NamespaceRegistry::with_common_prefixes().unwrap();
    let iri = registry.expand("schema:name").unwrap();
    assert_eq!(iri.as_str(), "https://schema.org/name");
}

#[test]
fn loads_people_fixture() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/people.json");
    let contents = std::fs::read_to_string(&path).unwrap();
    let graph = Graph::from_json(&contents).unwrap();
    assert_eq!(graph.len(), 1);
}

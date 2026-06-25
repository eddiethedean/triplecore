fn main() {
    use triplecore::{Graph, Iri, Literal, NamespaceRegistry};

    let mut graph = Graph::new();
    graph
        .add(
            Iri::new("https://example.com/person/1").unwrap(),
            Iri::new("https://schema.org/name").unwrap(),
            Literal::string("Ada Lovelace"),
        )
        .unwrap();

    println!("{}", graph.to_json_pretty().unwrap());

    let registry = NamespaceRegistry::with_common_prefixes().unwrap();
    println!("schema:name -> {}", registry.expand("schema:name").unwrap());
}

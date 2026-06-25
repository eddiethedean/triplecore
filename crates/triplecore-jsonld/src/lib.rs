//! JSON-LD writer (preview — full implementation planned for v0.2.0).

use triplecore_rdf::{Graph, RdfResult};

/// Export a graph as JSON-LD-compatible JSON.
///
/// v0.1.0 returns a minimal `@graph` wrapper. Full JSON-LD compaction arrives in v0.2.0.
pub fn to_jsonld(graph: &Graph) -> RdfResult<String> {
    let triples: Vec<_> = graph.iter().collect();
    let value = serde_json::json!({
        "@context": {
            "@vocab": "https://schema.org/"
        },
        "@graph": triples,
    });
    serde_json::to_string_pretty(&value).map_err(|err| {
        triplecore_rdf::RdfError::InvalidIri(format!("JSON-LD serialization failed: {err}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use triplecore_rdf::{Iri, Literal};

    #[test]
    fn exports_minimal_jsonld() {
        let mut graph = Graph::new();
        graph
            .add(
                Iri::new("https://example.com/person/1").unwrap(),
                Iri::new("https://schema.org/name").unwrap(),
                Literal::string("Ada Lovelace"),
            )
            .unwrap();
        let json = to_jsonld(&graph).unwrap();
        assert!(json.contains("@graph"));
    }
}

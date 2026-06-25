//! TripleCore — semantic runtime for ontology-native tools.
//!
//! This umbrella crate re-exports stable APIs from the component crates.

#![deny(missing_docs)]

/// RDF graph model and primitives.
#[cfg(feature = "rdf")]
pub use triplecore_rdf as rdf;

#[cfg(feature = "rdf")]
pub use triplecore_rdf::{
    BlankNode, Datatype, Dataset, Graph, GraphDiff, Iri, LanguageTag, Literal, NamespaceRegistry,
    Quad, RdfError, RdfResult, Term, Triple,
};

/// Shared diagnostics.
#[cfg(feature = "diagnostics")]
pub use triplecore_diagnostics as diagnostics;

#[cfg(feature = "diagnostics")]
pub use triplecore_diagnostics::{Diagnostic, DiagnosticCollection, Severity, SourceSpan};

/// Ontology entity model (preview).
#[cfg(feature = "ontology")]
pub use triplecore_ontology as ontology;

#[cfg(feature = "ontology")]
pub use triplecore_ontology::Ontology;

/// JSON-LD serialization (preview).
#[cfg(feature = "jsonld")]
pub use triplecore_jsonld as jsonld;

/// Turtle and N-Triples serialization (preview).
#[cfg(feature = "turtle")]
pub use triplecore_turtle as turtle;

/// Semantic mapping model (preview).
#[cfg(feature = "mapping")]
pub use triplecore_mapping as mapping;

/// Semantic query AST (preview).
#[cfg(feature = "query")]
pub use triplecore_query as query;

/// Planning and explain engine (preview).
#[cfg(feature = "planner")]
pub use triplecore_planner as planner;

/// Reasoning primitives (preview).
#[cfg(feature = "reason")]
pub use triplecore_reason as reason;

/// Crate version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reexports_graph_api() {
        let mut graph = Graph::new();
        graph
            .add(
                Iri::new("https://example.com/person/1").unwrap(),
                Iri::new("https://schema.org/name").unwrap(),
                Literal::string("Ada Lovelace"),
            )
            .unwrap();
        assert_eq!(graph.len(), 1);
    }
}

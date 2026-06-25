use serde::{Deserialize, Serialize};
use triplecore_rdf::{Graph, Iri};

/// Placeholder ontology document container.
///
/// Full class/property/individual modeling arrives in v0.3.0.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ontology {
    pub iri: Option<Iri>,
    pub graph: Graph,
}

impl Ontology {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_iri(iri: Iri) -> Self {
        Self {
            iri: Some(iri),
            graph: Graph::new(),
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }
}

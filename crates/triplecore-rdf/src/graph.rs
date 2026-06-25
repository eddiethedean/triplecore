use std::collections::HashSet;

use indexmap::IndexSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Iri, Literal, RdfResult, Term, Triple};

/// Result of comparing two graphs.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphDiff {
    pub added: Vec<Triple>,
    pub removed: Vec<Triple>,
}

impl GraphDiff {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}

/// An in-memory RDF graph backed by an insertion-ordered set of triples.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Graph {
    triples: IndexSet<Triple>,
}

impl Serialize for Graph {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.triples.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Graph {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let triples = IndexSet::<Triple>::deserialize(deserializer)?;
        Ok(Self { triples })
    }
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            triples: IndexSet::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.triples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.triples.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Triple> {
        self.triples.iter()
    }

    pub fn contains(&self, triple: &Triple) -> bool {
        self.triples.contains(triple)
    }

    pub fn insert(&mut self, triple: Triple) -> bool {
        self.triples.insert(triple)
    }

    pub fn add(
        &mut self,
        subject: impl Into<Term>,
        predicate: Iri,
        object: impl Into<Term>,
    ) -> RdfResult<bool> {
        Ok(self.insert(Triple::new(subject, predicate, object)))
    }

    pub fn add_iri_literal(
        &mut self,
        subject: Iri,
        predicate: Iri,
        object: Literal,
    ) -> RdfResult<bool> {
        self.add(subject, predicate, object)
    }

    pub fn remove(&mut self, triple: &Triple) -> bool {
        self.triples.swap_remove(triple)
    }

    pub fn extend<I: IntoIterator<Item = Triple>>(&mut self, triples: I) {
        for triple in triples {
            self.triples.insert(triple);
        }
    }

    pub fn merge(&mut self, other: &Graph) {
        self.extend(other.iter().cloned());
    }

    pub fn merged_with(&self, other: &Graph) -> Graph {
        let mut graph = self.clone();
        graph.merge(other);
        graph
    }

    pub fn diff(&self, other: &Graph) -> GraphDiff {
        let self_set: HashSet<&Triple> = self.triples.iter().collect();
        let other_set: HashSet<&Triple> = other.triples.iter().collect();

        let added = other
            .triples
            .iter()
            .filter(|t| !self_set.contains(t))
            .cloned()
            .collect();

        let removed = self
            .triples
            .iter()
            .filter(|t| !other_set.contains(t))
            .cloned()
            .collect();

        GraphDiff { added, removed }
    }

    pub fn to_json(&self) -> RdfResult<String> {
        serde_json::to_string(self).map_err(|err| {
            crate::RdfError::InvalidIri(format!("failed to serialize graph: {err}"))
        })
    }

    pub fn to_json_pretty(&self) -> RdfResult<String> {
        serde_json::to_string_pretty(self).map_err(|err| {
            crate::RdfError::InvalidIri(format!("failed to serialize graph: {err}"))
        })
    }

    pub fn from_json(json: &str) -> RdfResult<Self> {
        serde_json::from_str(json).map_err(|err| {
            crate::RdfError::InvalidIri(format!("failed to deserialize graph: {err}"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn person_graph() -> Graph {
        let mut graph = Graph::new();
        let subject = Iri::new("https://example.com/person/1").unwrap();
        let name = Iri::new("https://schema.org/name").unwrap();
        graph
            .add_iri_literal(subject.clone(), name, Literal::string("Ada Lovelace"))
            .unwrap();
        graph
    }

    #[test]
    fn adds_and_iterates_triples() {
        let graph = person_graph();
        assert_eq!(graph.len(), 1);
        assert!(!graph.is_empty());
    }

    #[test]
    fn merges_graphs() {
        let mut left = person_graph();
        let mut right = Graph::new();
        let subject = Iri::new("https://example.com/person/1").unwrap();
        let email = Iri::new("https://schema.org/email").unwrap();
        right
            .add_iri_literal(subject, email, Literal::string("ada@example.com"))
            .unwrap();
        left.merge(&right);
        assert_eq!(left.len(), 2);
    }

    #[test]
    fn diffs_graphs() {
        let left = person_graph();
        let mut right = person_graph();
        let subject = Iri::new("https://example.com/person/1").unwrap();
        let email = Iri::new("https://schema.org/email").unwrap();
        right
            .add_iri_literal(subject, email, Literal::string("ada@example.com"))
            .unwrap();
        let diff = left.diff(&right);
        assert_eq!(diff.added.len(), 1);
        assert!(diff.removed.is_empty());
    }

    #[test]
    fn round_trips_json() {
        let graph = person_graph();
        let json = graph.to_json_pretty().unwrap();
        let restored = Graph::from_json(&json).unwrap();
        assert_eq!(graph, restored);
    }
}

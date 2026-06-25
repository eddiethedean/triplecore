use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{Iri, RdfError, RdfResult};

/// Registry of namespace prefix → IRI expansions.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamespaceRegistry {
    prefixes: IndexMap<String, Iri>,
}

impl NamespaceRegistry {
    pub fn new() -> Self {
        Self {
            prefixes: IndexMap::new(),
        }
    }

    pub fn with_common_prefixes() -> RdfResult<Self> {
        let mut registry = Self::new();
        registry.register("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")?;
        registry.register("rdfs", "http://www.w3.org/2000/01/rdf-schema#")?;
        registry.register("xsd", "http://www.w3.org/2001/XMLSchema#")?;
        registry.register("schema", "https://schema.org/")?;
        Ok(registry)
    }

    pub fn register(&mut self, prefix: impl Into<String>, iri: impl AsRef<str>) -> RdfResult<()> {
        let prefix = prefix.into();
        validate_prefix(&prefix)?;
        let iri = Iri::new(iri.as_ref())?;
        self.prefixes.insert(prefix, iri);
        Ok(())
    }

    pub fn get(&self, prefix: &str) -> Option<&Iri> {
        self.prefixes.get(prefix)
    }

    pub fn expand(&self, prefixed: &str) -> RdfResult<Iri> {
        let (prefix, local) = prefixed
            .split_once(':')
            .ok_or_else(|| RdfError::InvalidIri(format!("not a prefixed name: {prefixed}")))?;

        let base = self.prefixes.get(prefix).ok_or_else(|| {
            RdfError::InvalidIri(format!("unknown prefix '{prefix}' in '{prefixed}'"))
        })?;

        Iri::new(format!("{}{local}", base.as_str()))
    }

    pub fn prefixes(&self) -> impl Iterator<Item = (&str, &Iri)> {
        self.prefixes.iter().map(|(k, v)| (k.as_str(), v))
    }

    pub fn len(&self) -> usize {
        self.prefixes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.prefixes.is_empty()
    }
}

fn validate_prefix(prefix: &str) -> RdfResult<()> {
    if prefix.is_empty() {
        return Err(RdfError::InvalidPrefix("prefix must not be empty".into()));
    }

    if prefix.contains(':') || prefix.chars().any(char::is_whitespace) {
        return Err(RdfError::InvalidPrefix(format!(
            "invalid prefix '{prefix}'"
        )));
    }

    Ok(())
}

/// Named graph container for quads grouped by graph IRI.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dataset {
    default: Vec<crate::Quad>,
    named: HashMap<Iri, Vec<crate::Quad>>,
}

impl Dataset {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, quad: crate::Quad) {
        match quad.graph.clone() {
            Some(graph) => self.named.entry(graph).or_default().push(quad),
            None => self.default.push(quad),
        }
    }

    pub fn default_graph(&self) -> &[crate::Quad] {
        &self.default
    }

    pub fn named_graph(&self, graph: &Iri) -> Option<&[crate::Quad]> {
        self.named.get(graph).map(Vec::as_slice)
    }

    pub fn graph_names(&self) -> impl Iterator<Item = &Iri> {
        self.named.keys()
    }

    pub fn len(&self) -> usize {
        self.default.len() + self.named.values().map(Vec::len).sum::<usize>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_prefixed_name() {
        let mut registry = NamespaceRegistry::new();
        registry
            .register("schema", "https://schema.org/")
            .unwrap();
        let iri = registry.expand("schema:name").unwrap();
        assert_eq!(iri.as_str(), "https://schema.org/name");
    }
}

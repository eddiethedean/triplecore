//! Reasoning primitives (preview — foundation for Ontologos).

use std::collections::{HashMap, HashSet};

use triplecore_rdf::Iri;

/// Simple subclass hierarchy for early inference helpers.
#[derive(Clone, Debug, Default)]
pub struct ClassHierarchy {
    parents: HashMap<Iri, HashSet<Iri>>,
}

impl ClassHierarchy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_subclass(&mut self, sub: Iri, super_: Iri) {
        self.parents.entry(sub).or_default().insert(super_);
    }

    pub fn ancestors(&self, class: &Iri) -> HashSet<Iri> {
        let mut seen = HashSet::new();
        let mut stack = vec![class.clone()];
        while let Some(current) = stack.pop() {
            if let Some(parents) = self.parents.get(&current) {
                for parent in parents {
                    if seen.insert(parent.clone()) {
                        stack.push(parent.clone());
                    }
                }
            }
        }
        seen
    }

    pub fn is_subclass_of(&self, sub: &Iri, super_: &Iri) -> bool {
        sub == super_ || self.ancestors(sub).contains(super_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_transitive_subclasses() {
        let mut hierarchy = ClassHierarchy::new();
        let person = Iri::new("https://schema.org/Person").unwrap();
        let agent = Iri::new("https://schema.org/Agent").unwrap();
        let thing = Iri::new("https://schema.org/Thing").unwrap();
        hierarchy.add_subclass(person.clone(), agent.clone());
        hierarchy.add_subclass(agent.clone(), thing.clone());
        assert!(hierarchy.is_subclass_of(&person, &thing));
    }
}

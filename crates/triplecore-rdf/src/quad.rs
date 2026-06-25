use serde::{Deserialize, Serialize};

use crate::{Iri, Term, Triple};

/// An RDF quad (triple plus optional graph name).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Quad {
    pub triple: Triple,
    pub graph: Option<Iri>,
}

impl Quad {
    pub fn new(triple: Triple, graph: Option<Iri>) -> Self {
        Self { triple, graph }
    }

    pub fn triple(triple: Triple) -> Self {
        Self {
            triple,
            graph: None,
        }
    }

    pub fn subject(&self) -> &Term {
        &self.triple.subject
    }

    pub fn predicate(&self) -> &Iri {
        &self.triple.predicate
    }

    pub fn object(&self) -> &Term {
        &self.triple.object
    }
}

impl From<Triple> for Quad {
    fn from(triple: Triple) -> Self {
        Self::triple(triple)
    }
}

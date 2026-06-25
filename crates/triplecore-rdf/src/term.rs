use serde::{Deserialize, Serialize};

use crate::{BlankNode, Iri, Literal};

/// An RDF term: IRI, blank node, or literal.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Term {
    Iri { iri: Iri },
    BlankNode { node: BlankNode },
    Literal { literal: Literal },
}

impl Term {
    pub fn iri(iri: Iri) -> Self {
        Self::Iri { iri }
    }

    pub fn blank_node(node: BlankNode) -> Self {
        Self::BlankNode { node }
    }

    pub fn literal(literal: Literal) -> Self {
        Self::Literal { literal }
    }

    pub fn is_iri(&self) -> bool {
        matches!(self, Self::Iri { .. })
    }

    pub fn is_blank_node(&self) -> bool {
        matches!(self, Self::BlankNode { .. })
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal { .. })
    }
}

impl From<Iri> for Term {
    fn from(iri: Iri) -> Self {
        Self::iri(iri)
    }
}

impl From<BlankNode> for Term {
    fn from(node: BlankNode) -> Self {
        Self::blank_node(node)
    }
}

impl From<Literal> for Term {
    fn from(literal: Literal) -> Self {
        Self::literal(literal)
    }
}

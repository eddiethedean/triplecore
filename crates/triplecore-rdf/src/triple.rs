use serde::{Deserialize, Serialize};

use crate::{Iri, Literal, Term};

/// An RDF triple (subject, predicate, object).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Triple {
    pub subject: Term,
    pub predicate: Iri,
    pub object: Term,
}

impl Triple {
    pub fn new(subject: impl Into<Term>, predicate: Iri, object: impl Into<Term>) -> Self {
        Self {
            subject: subject.into(),
            predicate,
            object: object.into(),
        }
    }
}

/// Convenience constructors using IRIs for subject and object.
impl Triple {
    pub fn iri(subject: Iri, predicate: Iri, object: Iri) -> Self {
        Self::new(subject, predicate, object)
    }

    pub fn iri_literal(subject: Iri, predicate: Iri, object: Literal) -> Self {
        Self::new(subject, predicate, object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_triple() {
        let s = Iri::new("https://example.com/person/1").unwrap();
        let p = Iri::new("https://schema.org/name").unwrap();
        let o = Literal::string("Ada Lovelace");
        let triple = Triple::new(s.clone(), p.clone(), o);
        assert_eq!(triple.subject, Term::iri(s));
        assert_eq!(triple.predicate, p);
    }
}

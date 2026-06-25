//! Core RDF primitives: IRIs, literals, triples, graphs, and namespace registries.

mod blank_node;
mod error;
mod graph;
mod iri;
mod literal;
mod namespace;
mod quad;
mod term;
mod triple;

pub use blank_node::BlankNode;
pub use error::{RdfError, RdfResult};
pub use graph::{Graph, GraphDiff};
pub use iri::Iri;
pub use literal::{Datatype, LanguageTag, Literal};
pub use namespace::{Dataset, NamespaceRegistry};
pub use quad::Quad;
pub use term::Term;
pub use triple::Triple;

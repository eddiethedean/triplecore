# ADR 0008: External Crate Dependencies

## Status

Accepted

## Context

TripleCore v0.1.0 introduced a native RDF model to move quickly. The broader plan originally deferred all external RDF libraries to avoid locking into another data model early.

However, reimplementing Turtle, JSON-LD, N-Triples, IRI handling, and SPARQL parsing is high-risk for conformance bugs and maintenance cost. The Rust ecosystem now has a cohesive, actively maintained stack from the Oxigraph project.

## Decision

Adopt **proven Rust crates at internal boundaries**, wrapped by TripleCore adapter layers:

- **RDF model & I/O**: `oxrdf`, `oxiri`, `oxilangtag`, `oxttl`, `oxjsonld`, `oxrdfio`
- **Schema validation**: `jsonschema`, `serde_yaml`
- **SPARQL parse-only**: `spargebra` (not execution)
- **Graph algorithms**: `petgraph`
- **Bindings**: `pyo3`, `wasm-bindgen`

TripleCore continues to own:

- Public API surface for bindings
- Semantic mapping and query AST
- Planner, explain output, diagnostics codes
- Ontology entity index and projection
- Cross-language fixtures and schemas

Do **not** adopt:

- Full `oxigraph` store (execution stays in SparqlModel)
- Deprecated `rio` crates
- `sophia` unless a concrete adapter need arises

## Consequences

- v0.2.0 introduces adapter migration from native types to `oxrdf` internals
- MSRV likely rises to match Oxigraph crates (~1.87)
- Conformance tests can leverage W3C fixtures via `oxrdfio` parsers
- Less custom serialization code to maintain
- Public API stability remains a TripleCore concern; internal crate swaps are encapsulated

## References

- [docs/dependencies.md](../dependencies.md)
- [OxRDF](https://docs.rs/oxrdf)
- [OxRDF I/O](https://docs.rs/oxrdfio)
- [Spargebra](https://docs.rs/spargebra)

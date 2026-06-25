# Roadmap

## v0.1.0 — Graph Core (current)

- [x] RDF node model (IRI, blank node, literal)
- [x] Triples and quads
- [x] Graph with merge and diff
- [x] Namespace registry
- [x] JSON serialization
- [x] Workspace skeleton and CI

## v0.2.0 — Serialization

- JSON-LD writer with deterministic output
- N-Triples and Turtle writers
- Snapshot tests

## v0.3.0 — Ontology Model

- Class, property, individual entities
- Labels, comments, definitions
- Entity index and inspect API

## v0.4.0 — Mapping Model

- Entity and property mappings
- Source paths and identity templates
- Validation diagnostics

## v0.5.0 — CLI

- validate, inspect, convert, explain, diff, namespaces

## v0.6.0 — Python Bindings

- PyO3 + maturin
- TripleModel prototype integration

## v0.7.0 — WASM Bindings

- wasm-bindgen + npm packages
- OntoEagle prototype integration

## v0.8.0 — Query AST

- Semantic select, filters, paths, serialization

## v0.9.0 — Planner

- Mapping-aware query planning
- Explain plans for OntoSQL

## v1.0.0 — Stable Runtime

- Stable API surface
- Production-ready Python and WASM bindings
- Ecosystem integration examples

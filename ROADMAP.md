# Roadmap

Dependency targets for each release are documented in [docs/dependencies.md](docs/dependencies.md).

## v0.1.0 — Graph Core (current)

- [x] RDF node model (IRI, blank node, literal)
- [x] Triples and quads
- [x] Graph with merge and diff
- [x] Namespace registry
- [x] JSON serialization
- [x] Workspace skeleton and CI
- **Deps**: `serde`, `indexmap`, `thiserror`, `url`, `clap`, `schemars`

## v0.2.0 — Serialization

- Migrate graph storage to [`oxrdf`](https://crates.io/crates/oxrdf) via internal adapters
- Turtle / N-Triples / N-Quads via [`oxttl`](https://crates.io/crates/oxttl)
- JSON-LD via [`oxjsonld`](https://crates.io/crates/oxjsonld)
- Unified CLI I/O via [`oxrdfio`](https://crates.io/crates/oxrdfio)
- IRI and language tags via [`oxiri`](https://crates.io/crates/oxiri), [`oxilangtag`](https://crates.io/crates/oxilangtag)
- Deterministic output and [`insta`](https://crates.io/crates/insta) snapshot tests

## v0.3.0 — Ontology Model

- Class, property, individual entities
- Labels, comments, definitions
- Entity index and inspect API
- **Deps**: builds on `oxrdf` graph substrate (no new major crates)

## v0.4.0 — Mapping Model

- Entity and property mappings
- Source paths and identity templates
- Validation diagnostics
- **Deps**: [`jsonschema`](https://crates.io/crates/jsonschema), [`serde_yaml`](https://crates.io/crates/serde_yaml)

## v0.5.0 — CLI

- validate, inspect, convert, explain, diff, namespaces
- RDF/XML support via [`oxrdfxml`](https://crates.io/crates/oxrdfxml) (optional feature)
- Fixture-based tests through `oxrdfio` parsers

## v0.6.0 — Python Bindings

- PyO3 + maturin ([`pyo3`](https://crates.io/crates/pyo3), [`pyo3-stub-gen`](https://crates.io/crates/pyo3-stub-gen))
- Python package and pytest suite
- TripleModel prototype integration

## v0.7.0 — WASM Bindings

- wasm-bindgen + npm packages
- TypeScript declarations
- Browser example (OntoEagle prototype)

## v0.8.0 — Query AST

- Semantic select, filters, paths, serialization
- SPARQL parse/inspect helpers via [`spargebra`](https://crates.io/crates/spargebra) — adapters only, not execution

## v0.9.0 — Planner

- Explain plans and mapping-aware query planning
- Path planning via [`petgraph`](https://crates.io/crates/petgraph)
- OntoSQL prototype integration

## v1.0.0 — Stable Runtime

- Stable API surface
- Production-ready Python and WASM bindings
- Ecosystem integration examples
- Release automation

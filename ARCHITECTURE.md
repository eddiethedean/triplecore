# Architecture

TripleCore is organized as a Rust workspace with layered crates and future language bindings.

```text
                         TripleCore Monorepo
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
      Rust                 Python              TypeScript/WASM
        │                     │                     │
   native crates        PyO3/maturin          wasm-bindgen/npm
        │                     │                     │
   CLI + reasoner       TripleModel           OntoEagle/OntoCode
                        OntoSQL
                        SparqlModel
```

## Crate layers (v0.1.0)

| Crate | Status | Responsibility |
|-------|--------|----------------|
| `triplecore-rdf` | **Stable preview** | IRI, literal, triple, graph, namespace registry |
| `triplecore-diagnostics` | **Stable preview** | Error codes, severity, JSON diagnostics |
| `triplecore-jsonld` | Preview | Minimal JSON-LD export |
| `triplecore-turtle` | Preview | N-Triples writer |
| `triplecore-ontology` | Preview | Ontology document container |
| `triplecore-mapping` | Preview | Entity/property mapping model |
| `triplecore-query` | Preview | Semantic query AST |
| `triplecore-planner` | Preview | Explain plans |
| `triplecore-reason` | Preview | Class hierarchy helpers |
| `triplecore-cli` | Preview | CLI tool |

## Design principles

- **Minimal, strongly typed APIs** — binding-friendly and deterministic
- **Schemas as contracts** — portable interchange for tools and AI assistants
- **Incremental delivery** — one crate at a time, shared fixture suite later
- **No premature dependencies** — avoid locking into external RDF library data models early

See [docs/decisions/](docs/decisions/) for architecture decision records.

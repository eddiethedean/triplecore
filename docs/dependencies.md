# Dependency Strategy

TripleCore owns the **semantic runtime API** (mapping model, query AST, planner, diagnostics, bindings). It delegates **standards-heavy, conformance-critical** work to mature Rust crates and wraps them behind stable adapter layers.

See [ADR 0008: External crate dependencies](decisions/0008-external-crate-dependencies.md).

## Principles

1. **Do not reimplement RDF syntax or SPARQL parsing** — use the Oxigraph ecosystem.
2. **Keep TripleCore types binding-friendly** — thin adapters between public API and internal crates.
3. **Pin Oxigraph-family versions together** — `oxrdf`, `oxttl`, `oxjsonld`, and `oxrdfio` release in lockstep.
4. **Own what is unique** — semantic mappings, explain plans, ontology entity index, cross-language fixtures.
5. **Avoid deprecated crates** — `rio`/`rio_*` are unmaintained; use `oxrdfio` / `oxttl` instead.

## Dependency tiers

### Tier 1 — Workspace foundations (now)

| Crate | Use in TripleCore |
|-------|-------------------|
| [`serde`](https://crates.io/crates/serde) / [`serde_json`](https://crates.io/crates/serde_json) | JSON interchange, binding payloads |
| [`thiserror`](https://crates.io/crates/thiserror) / [`anyhow`](https://crates.io/crates/anyhow) | Library vs CLI error handling |
| [`indexmap`](https://crates.io/crates/indexmap) | Deterministic insertion order |
| [`url`](https://crates.io/crates/url) / [`oxiri`](https://crates.io/crates/oxiri) | IRI validation and resolution |
| [`clap`](https://crates.io/crates/clap) | CLI |
| [`schemars`](https://crates.io/crates/schemars) | JSON Schema generation from Rust types |

### Tier 2 — RDF core (v0.2.0+)

| Crate | Use in TripleCore | TripleCore crate |
|-------|-------------------|------------------|
| [`oxrdf`](https://crates.io/crates/oxrdf) | Internal graph/term storage, RDF 1.1/1.2 types | `triplecore-rdf` |
| [`oxiri`](https://crates.io/crates/oxiri) | IRI parsing and normalization | `triplecore-rdf` |
| [`oxilangtag`](https://crates.io/crates/oxilangtag) | BCP47 language tags | `triplecore-rdf` |
| [`oxttl`](https://crates.io/crates/oxttl) | Turtle, TriG, N-Triples, N-Quads read/write | `triplecore-turtle` |
| [`oxjsonld`](https://crates.io/crates/oxjsonld) | JSON-LD read/write | `triplecore-jsonld` |
| [`oxrdfio`](https://crates.io/crates/oxrdfio) | Unified RDF format dispatch for CLI `convert` | `triplecore-cli` |
| [`oxrdfxml`](https://crates.io/crates/oxrdfxml) | RDF/XML (optional, CLI only initially) | `triplecore-cli` |

v0.1.0 ships a lightweight native RDF model for fast iteration. **v0.2.0 migrates storage and serialization to Oxigraph crates** while preserving the public `triplecore::Graph` API via adapters.

### Tier 3 — Validation and config (v0.4.0+)

| Crate | Use in TripleCore | TripleCore crate |
|-------|-------------------|------------------|
| [`jsonschema`](https://crates.io/crates/jsonschema) | Validate artifacts against `schemas/*.schema.json` | `triplecore-diagnostics`, CLI |
| [`serde_yaml`](https://crates.io/crates/serde_yaml) | Parse mapping/config YAML | `triplecore-mapping`, CLI |
| [`regex`](https://crates.io/crates/regex) | Source-path and template validation | `triplecore-mapping` |

### Tier 4 — Query and graph algorithms (v0.8.0+)

| Crate | Use in TripleCore | TripleCore crate |
|-------|-------------------|------------------|
| [`spargebra`](https://crates.io/crates/spargebra) | Parse SPARQL for inspection and adapter codegen — **not execution** | `triplecore-query` (adapter helpers) |
| [`petgraph`](https://crates.io/crates/petgraph) | Path planning, hierarchy traversal | `triplecore-planner`, `triplecore-reason` |

TripleCore defines a **semantic query AST**. SparqlModel and OntoSQL compile it to SPARQL/SQL. Spargebra is for parsing/inspecting SPARQL strings, not replacing downstream executors.

### Tier 5 — Bindings (v0.6.0 / v0.7.0)

| Crate | Use in TripleCore |
|-------|-------------------|
| [`pyo3`](https://crates.io/crates/pyo3) / [`pyo3-stub-gen`](https://crates.io/crates/pyo3-stub-gen) | Python extension + type stubs |
| [`maturin`](https://crates.io/crates/maturin) | Python wheel build |
| [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen) | WASM exports |
| [`wasm-bindgen-futures`](https://crates.io/crates/wasm-bindgen-futures) | Async browser integration (OntoEagle) |
| [`console_error_panic_hook`](https://crates.io/crates/console_error_panic_hook) | WASM debug |

### Tier 6 — Testing and DX

| Crate | Use |
|-------|-----|
| [`insta`](https://crates.io/crates/insta) | Snapshot tests for serialization and CLI output |
| [`pretty_assertions`](https://crates.io/crates/pretty_assertions) | Readable test diffs |

## Explicitly not in scope

| Crate | Reason |
|-------|--------|
| **`oxigraph`** (full store) | TripleCore is a runtime layer, not a database. SparqlModel/OntoSQL keep execution. |
| **`sophia`** | Overlapping abstractions; Oxigraph ecosystem is better maintained and aligns with conformance tooling. Revisit only if a Sophia-specific adapter is needed. |
| **`rio` / `rio_turtle` / `rio_xml`** | Deprecated; replaced by `oxrdfio` / `oxttl`. |

## Version roadmap mapping

| Release | New / upgraded dependencies |
|---------|------------------------------|
| **0.1.0** | `serde`, `indexmap`, `thiserror`, `url`, `clap`, `schemars` |
| **0.2.0** | `oxrdf`, `oxiri`, `oxilangtag`, `oxttl`, `oxjsonld`, `oxrdfio`, `insta` |
| **0.3.0** | (builds on `oxrdf` graph substrate) |
| **0.4.0** | `jsonschema`, `serde_yaml` |
| **0.5.0** | `oxrdfxml` (optional RDF/XML in CLI) |
| **0.6.0** | `pyo3`, `pyo3-stub-gen` |
| **0.7.0** | `wasm-bindgen`, `console_error_panic_hook` |
| **0.8.0** | `spargebra` (parse-only helpers) |
| **0.9.0** | `petgraph` |

## Adapter pattern

```text
Public API (triplecore::Graph, Iri, Literal)
        │
        ▼
Adapter layer (From/Into, #[doc(hidden)] internals)
        │
        ▼
Oxigraph crates (oxrdf::Graph, NamedNode, Literal, …)
```

Bindings (Python/WASM) only see the public API. Adapters are internal and can evolve as upstream crates release.

## MSRV note

Oxigraph-family crates currently require **Rust 1.87+**. When adopting them in v0.2.0, bump `rust-version` in the workspace and CI toolchain accordingly.

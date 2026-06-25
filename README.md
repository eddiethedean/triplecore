# TripleCore

**TripleCore** is a Rust-first semantic runtime monorepo for building ontology-native tools across Python, TypeScript, WebAssembly, native CLIs, and future applications.

Version **0.1.0** delivers the **Graph Core**: RDF node model, triples, graphs, namespace registries, and JSON serialization.

## Quick start

```bash
# Build and test
cargo test

# Run the CLI
cargo run -p triplecore-cli -- version
```

### Rust API

```rust
use triplecore::{Graph, Iri, Literal};

let mut graph = Graph::new();

graph.add(
    Iri::new("https://example.com/person/1")?,
    Iri::new("https://schema.org/name")?,
    Literal::string("Ada Lovelace"),
)?;

println!("{}", graph.to_json_pretty()?);
```

## Workspace layout

```text
crates/
├── triplecore/           # Umbrella crate
├── triplecore-rdf/       # RDF graph model (v0.1.0)
├── triplecore-diagnostics/
├── triplecore-jsonld/    # Preview JSON-LD export
├── triplecore-turtle/    # Preview N-Triples export
├── triplecore-ontology/  # Preview ontology container
├── triplecore-mapping/   # Preview mapping model
├── triplecore-query/     # Preview semantic query AST
├── triplecore-planner/   # Preview explain plans
├── triplecore-reason/    # Preview class hierarchy helpers
└── triplecore-cli/       # CLI tool
```

## Development

```bash
just fmt     # Format Rust code
just lint    # Run clippy
just test    # Run all Rust tests
just build   # Build workspace
```

## Roadmap

| Version | Focus |
|---------|-------|
| 0.1.0 | Graph Core (this release) |
| 0.2.0 | JSON-LD, Turtle, N-Triples writers |
| 0.3.0 | Ontology entity model |
| 0.4.0 | Mapping model and validation |
| 0.5.0 | CLI commands |
| 0.6.0 | Python bindings |
| 0.7.0 | WASM bindings |

See [ROADMAP.md](ROADMAP.md) for the full plan.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

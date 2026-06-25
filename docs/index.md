# TripleCore

TripleCore is a Rust-first semantic runtime for ontology-native tools.

## Install (Rust)

```bash
cargo add triplecore
```

## v0.1.0 — Graph Core

See the [getting started guide](getting-started.md) and [RDF model](rdf-model.md) documentation.

## Dependencies

TripleCore delegates RDF syntax and conformance work to the [Oxigraph ecosystem](https://github.com/oxigraph/oxigraph) (`oxrdf`, `oxttl`, `oxjsonld`, …) behind stable adapter layers. Full dependency plan: [dependencies.md](dependencies.md).

## Bindings

Python and WASM bindings are planned for v0.6.0 and v0.7.0 respectively.

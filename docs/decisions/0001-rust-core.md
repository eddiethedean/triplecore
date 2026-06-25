# ADR 0001: Rust Core

## Status

Accepted

## Context

Multiple ontology tools (TripleModel, OntoSQL, OntoEagle, OntoCode, Ontologos) need shared semantic logic.

## Decision

Implement the core runtime in Rust with language bindings for Python and WASM.

## Consequences

- Single high-performance implementation
- PyO3 and wasm-bindgen required for bindings
- Rust expertise needed for core changes

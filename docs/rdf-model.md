# RDF Model

TripleCore v0.1.0 provides an in-memory RDF graph model in the `triplecore-rdf` crate (re-exported by `triplecore`).

**v0.2.0 plan:** migrate internal storage to [`oxrdf`](https://crates.io/crates/oxrdf) while keeping the public API stable. See [dependencies.md](dependencies.md).

## Core types

| Type | Description |
|------|-------------|
| `Iri` | Validated IRI string |
| `BlankNode` | Anonymous node (`_:id`) |
| `Literal` | Plain, language-tagged, or typed literal |
| `Term` | IRI, blank node, or literal |
| `Triple` | Subject, predicate, object |
| `Graph` | Insertion-ordered set of triples |
| `NamespaceRegistry` | Prefix → IRI expansion |

## Graph operations

```rust
use triplecore::{Graph, GraphDiff, Iri, Literal};

let mut graph = Graph::new();
// add triples, merge other graphs, compute diffs
let diff: GraphDiff = graph.diff(&other);
```

## JSON serialization

Graphs serialize to JSON via `to_json`, `to_json_pretty`, and `from_json`.

## Namespace registry

```rust
use triplecore::NamespaceRegistry;

let registry = NamespaceRegistry::with_common_prefixes()?;
let name = registry.expand("schema:name")?;
```

Common prefixes: `rdf`, `rdfs`, `xsd`, `schema`.

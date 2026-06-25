# Vision

TripleCore is the **semantic runtime layer** for a complete ontology engineering platform.

It is not just a Rust library — it is the shared foundation that lets Python, TypeScript, WASM, and CLI tools operate on the same RDF graph model, ontology entities, mappings, queries, and diagnostics.

## Downstream projects

| Project | Role |
|---------|------|
| **TripleModel** | Python semantic object modeling |
| **OntoSQL** | SQL/relational semantic mapping |
| **SparqlModel** | SPARQL/graph semantic mapping |
| **OntoEagle** | Browser-based ontology exploration |
| **OntoCode** | VS Code ontology IDE |
| **Ontologos** | Rust-native reasoning and inference |

Each project keeps its identity. TripleCore provides the common substrate:

- RDF graph model
- Ontology entity model
- Mapping model
- Query AST
- Validation diagnostics
- Serialization
- Explain plans
- Runtime bindings

## Success criteria

TripleCore succeeds when:

- TripleModel can delegate graph primitives without losing Python ergonomics
- OntoSQL can expose mappings to OntoEagle without duplicating logic in TypeScript
- OntoEagle can run semantic logic in the browser through WASM
- OntoCode can display diagnostics from the same engine
- One fixture suite validates behavior across Rust, Python, and TypeScript
- Users can inspect, validate, convert, and explain semantic artifacts from one CLI

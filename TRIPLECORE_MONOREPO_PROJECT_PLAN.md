# TripleCore Monorepo Project Plan

## Executive Summary

**TripleCore** is a Rust-first semantic runtime monorepo for building ontology-native tools across Python, TypeScript, WebAssembly, native CLIs, and future applications.

The project should provide the shared foundation for:

- **TripleModel** — Python semantic object modeling
- **OntoSQL** — SQL/relational semantic mapping
- **SparqlModel** — SPARQL/graph semantic mapping
- **OntoEagle** — browser-based ontology exploration
- **OntoCode** — VS Code ontology IDE
- **Ontologos** — Rust-native reasoning and inference

The goal is to avoid duplicated semantic logic across projects by creating one reusable, tested, high-performance core.

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

---

## Core Product Thesis

TripleCore is not just a Rust library.

It is the **semantic runtime layer** for a complete ontology engineering platform.

Each downstream project keeps its identity:

```text
TripleModel  = Python semantic model layer
OntoSQL      = relational database adapter
SparqlModel  = graph database adapter
OntoEagle    = browser explorer
OntoCode     = IDE layer
Ontologos    = reasoning layer
```

TripleCore provides the common foundation:

```text
RDF graph model
ontology entity model
mapping model
query AST
validation diagnostics
serialization
explain plans
runtime bindings
```

---

## Repository Name

Recommended repository name:

```text
triplecore
```

Recommended package names:

```text
Rust crate:        triplecore
Python package:    triplecore
NPM package:       @triplecore/core
WASM package:      @triplecore/wasm
CLI command:       triplecore
```

---

## Monorepo Layout

```text
triplecore/
├── README.md
├── VISION.md
├── ARCHITECTURE.md
├── ROADMAP.md
├── SPEC.md
├── CONTRIBUTING.md
├── LICENSE
├── CHANGELOG.md
├── Cargo.toml
├── rust-toolchain.toml
├── justfile
├── Makefile
├── package.json
├── pyproject.toml
├── uv.lock
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── rust.yml
│       ├── python.yml
│       ├── wasm.yml
│       ├── docs.yml
│       └── release.yml
├── crates/
│   ├── triplecore/
│   ├── triplecore-rdf/
│   ├── triplecore-ontology/
│   ├── triplecore-jsonld/
│   ├── triplecore-turtle/
│   ├── triplecore-mapping/
│   ├── triplecore-query/
│   ├── triplecore-planner/
│   ├── triplecore-diagnostics/
│   ├── triplecore-reason/
│   ├── triplecore-py/
│   ├── triplecore-wasm/
│   └── triplecore-cli/
├── python/
│   ├── triplecore/
│   └── tests/
├── js/
│   ├── packages/
│   │   ├── core/
│   │   ├── wasm/
│   │   └── react/
│   └── examples/
├── examples/
│   ├── rust/
│   ├── python/
│   ├── node/
│   ├── browser/
│   ├── ontoeagle/
│   └── ontosql/
├── docs/
│   ├── index.md
│   ├── getting-started.md
│   ├── rdf-model.md
│   ├── ontology-model.md
│   ├── mappings.md
│   ├── queries.md
│   ├── python-bindings.md
│   ├── wasm-bindings.md
│   ├── cli.md
│   ├── architecture/
│   └── decisions/
├── schemas/
│   ├── mapping.schema.json
│   ├── ontology.schema.json
│   ├── query.schema.json
│   └── diagnostics.schema.json
├── benches/
│   ├── graph_load.rs
│   ├── serialization.rs
│   └── mapping_validation.rs
└── tests/
    ├── fixtures/
    ├── conformance/
    ├── snapshots/
    └── integration/
```

---

## Workspace Strategy

TripleCore should use a Rust workspace at the root.

Root `Cargo.toml`:

```toml
[workspace]
members = [
    "crates/triplecore",
    "crates/triplecore-rdf",
    "crates/triplecore-ontology",
    "crates/triplecore-jsonld",
    "crates/triplecore-turtle",
    "crates/triplecore-mapping",
    "crates/triplecore-query",
    "crates/triplecore-planner",
    "crates/triplecore-diagnostics",
    "crates/triplecore-reason",
    "crates/triplecore-py",
    "crates/triplecore-wasm",
    "crates/triplecore-cli",
]
resolver = "2"

[workspace.package]
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/YOUR_ORG/triplecore"
homepage = "https://github.com/YOUR_ORG/triplecore"
documentation = "https://triplecore.readthedocs.io"
```

---

## Crate Architecture

## `triplecore-rdf`

Core RDF primitives.

Responsibilities:

- IRI
- blank node
- literal
- datatype
- language tag
- triple
- quad
- graph
- dataset
- namespace registry
- graph operations
- graph diff
- graph merge

Public API examples:

```rust
use triplecore_rdf::{Graph, Iri, Literal};

let mut graph = Graph::new();

graph.add(
    Iri::new("https://example.com/person/1")?,
    Iri::new("https://schema.org/name")?,
    Literal::string("Ada Lovelace"),
)?;
```

---

## `triplecore-ontology`

Ontology entity model.

Responsibilities:

- ontology document
- class
- object property
- datatype property
- annotation property
- individual
- imports
- labels
- comments
- definitions
- entity lookup
- entity graph projection

Public concepts:

```text
Ontology
OntologyEntity
Class
Property
Individual
Annotation
OntologyIndex
```

---

## `triplecore-jsonld`

JSON-LD support.

Responsibilities:

- JSON-LD writer
- JSON-LD context model
- compact output
- expanded output
- deterministic output
- browser-friendly serialization
- integration with OntoEagle static datasets

Initial goal:

```text
Graph → JSON-LD
Ontology → JSON-LD
Mapping → JSON-LD-friendly representation
```

Defer full JSON-LD processor implementation until later.

---

## `triplecore-turtle`

RDF text format support.

Responsibilities:

- N-Triples writer
- Turtle writer
- prefix handling
- deterministic output
- later parser support

Initial goal:

```text
Graph → N-Triples
Graph → Turtle
```

Later:

```text
Turtle → Graph
N-Triples → Graph
```

---

## `triplecore-mapping`

Language-independent semantic mapping model.

This crate is the bridge between:

- SQL rows
- SPARQL results
- JSON documents
- CSV records
- API payloads
- ontology entities

Responsibilities:

- entity mapping
- property mapping
- identity templates
- source paths
- target IRIs
- relationship mapping
- cardinality
- nullability
- datatype conversion hints
- transform metadata
- mapping validation hooks

Example model:

```text
EntityMapping
├── name
├── class_iri
├── source
├── identity
└── properties

PropertyMapping
├── property_iri
├── source_path
├── target_kind
├── datatype
├── cardinality
├── required
└── relationship
```

---

## `triplecore-query`

Portable semantic query AST.

Responsibilities:

- semantic select
- filters
- paths
- relationship traversal
- ordering
- limit/offset
- variables
- query serialization
- query validation

This should not execute SQL or SPARQL directly.

It should define a common query language that adapters compile.

```text
SemanticQuery → OntoSQL SQL plan
SemanticQuery → SparqlModel SPARQL query
SemanticQuery → OntoEagle visualization
SemanticQuery → in-memory graph traversal
```

---

## `triplecore-planner`

Planning and explain engine.

Responsibilities:

- mapping-aware path planning
- query planning
- relationship expansion
- projection planning
- generated plan model
- explain output
- warning collection
- validation coordination

Plan types:

```text
MappingPlan
QueryPlan
ProjectionPlan
ExplainPlan
PathPlan
JoinPlan
```

OntoSQL uses this for relational mapping.

SparqlModel uses this for graph query mapping.

OntoEagle uses this for visualization.

OntoCode uses this for diagnostics.

---

## `triplecore-diagnostics`

Shared diagnostics system.

Responsibilities:

- error codes
- warning codes
- source spans
- severity levels
- fix suggestions
- JSON output
- LSP-friendly diagnostics
- CLI-friendly diagnostics

Severity:

```text
Error
Warning
Info
Hint
```

Example diagnostic:

```json
{
  "code": "TC-MAP-001",
  "severity": "error",
  "message": "Property mapping references an unknown source field.",
  "target": "Person.name",
  "suggestion": "Check the source path or schema metadata."
}
```

---

## `triplecore-reason`

Future reasoning primitives.

Initial scope should be intentionally small.

Responsibilities in early versions:

- class hierarchy traversal
- property hierarchy traversal
- transitive closure helper
- simple subclass inference
- simple domain/range checks

Defer:

- full OWL DL reasoning
- tableau algorithms
- ELK-style classification
- HermiT replacement
- explanation engine

This crate can later become the foundation for **Ontologos**.

---

## `triplecore`

Umbrella crate.

Responsibilities:

- re-export stable APIs
- provide simple entry point
- hide internal crate boundaries
- expose common feature flags

Example:

```rust
use triplecore::{Graph, Iri, Literal, Ontology, EntityMapping};
```

Feature flags:

```toml
[features]
default = ["rdf", "ontology", "jsonld", "mapping", "query"]
rdf = ["triplecore-rdf"]
ontology = ["triplecore-ontology"]
jsonld = ["triplecore-jsonld"]
turtle = ["triplecore-turtle"]
mapping = ["triplecore-mapping"]
query = ["triplecore-query"]
planner = ["triplecore-planner"]
reason = ["triplecore-reason"]
```

---

## `triplecore-py`

Python bindings.

Technology:

```text
PyO3 + maturin
```

Responsibilities:

- expose RDF primitives to Python
- expose graph operations
- expose ontology model
- expose mapping validation
- expose JSON-LD export
- expose query AST
- expose explain plans
- provide Pythonic wrappers

Install:

```bash
pip install triplecore
```

Example:

```python
from triplecore import Graph, Iri, Literal

g = Graph()
g.add(
    Iri("https://example.com/person/1"),
    Iri("https://schema.org/name"),
    Literal("Ada Lovelace"),
)

print(g.to_jsonld())
```

Downstream consumers:

```text
TripleModel
OntoSQL
SparqlModel
Ontologos
```

---

## `triplecore-wasm`

WebAssembly bindings.

Technology:

```text
wasm-bindgen + wasm-pack
```

Responsibilities:

- browser-compatible graph model
- JSON-LD export
- ontology entity lookup
- mapping validation
- query plan visualization data
- explain plans
- TypeScript declarations

Install:

```bash
npm install @triplecore/wasm
```

Example:

```ts
import { Graph, Iri, Literal } from "@triplecore/wasm";

const g = new Graph();

g.add(
  new Iri("https://example.com/person/1"),
  new Iri("https://schema.org/name"),
  Literal.string("Ada Lovelace")
);

console.log(g.toJsonLd());
```

Downstream consumers:

```text
OntoEagle
OntoCode
browser demos
Node tools
```

---

## `triplecore-cli`

Command-line tool.

Install:

```bash
cargo install triplecore-cli
```

Commands:

```bash
triplecore validate ontology.ttl
triplecore inspect ontology.ttl
triplecore convert input.ttl --to jsonld
triplecore explain mapping.yaml
triplecore query graph.ttl --select Person.name
triplecore diff old.ttl new.ttl
triplecore namespaces ontology.ttl
```

The CLI should become the universal semantic debugging tool.

---

## Python Package Layout

```text
python/
├── triplecore/
│   ├── __init__.py
│   ├── graph.py
│   ├── ontology.py
│   ├── mapping.py
│   ├── query.py
│   ├── diagnostics.py
│   └── py.typed
└── tests/
    ├── test_graph.py
    ├── test_jsonld.py
    ├── test_mapping.py
    ├── test_query.py
    └── test_diagnostics.py
```

The Python package should be thin.

It should wrap the Rust extension and provide ergonomic Python APIs.

---

## JavaScript Package Layout

```text
js/
├── package.json
├── packages/
│   ├── wasm/
│   │   ├── package.json
│   │   ├── src/
│   │   └── generated/
│   ├── core/
│   │   ├── package.json
│   │   └── src/
│   └── react/
│       ├── package.json
│       └── src/
└── examples/
    ├── vite-browser/
    ├── node-cli/
    └── ontoeagle-prototype/
```

Recommended packages:

```text
@triplecore/wasm
@triplecore/core
@triplecore/react
```

`@triplecore/react` should be optional and later-stage.

It can provide components like:

```text
GraphViewer
OntologyEntityPanel
MappingExplainPanel
DiagnosticsPanel
```

---

## Schema Files

TripleCore should define JSON schemas for portable interchange.

```text
schemas/
├── graph.schema.json
├── ontology.schema.json
├── mapping.schema.json
├── query.schema.json
├── explain.schema.json
└── diagnostics.schema.json
```

These schemas matter because they allow:

- OntoEagle to consume mappings
- OntoCode to validate project files
- Cursor/Claude/ChatGPT to generate valid configs
- Python and TS to share test fixtures
- CLI to validate user-authored files

---

## Documentation Plan

Initial docs:

```text
README.md
VISION.md
ARCHITECTURE.md
ROADMAP.md
SPEC.md
CONTRIBUTING.md
```

Detailed docs:

```text
docs/
├── getting-started.md
├── install.md
├── rdf-model.md
├── ontology-model.md
├── mapping-model.md
├── semantic-query.md
├── planner.md
├── diagnostics.md
├── jsonld.md
├── turtle.md
├── python-bindings.md
├── typescript-bindings.md
├── wasm.md
├── cli.md
├── examples.md
├── integration-ontomodel.md
├── integration-ontosql.md
├── integration-sparqlmodel.md
├── integration-ontoeagle.md
├── integration-ontocode.md
└── integration-ontologos.md
```

Architecture decision records:

```text
docs/decisions/
├── 0001-rust-core.md
├── 0002-pyo3-bindings.md
├── 0003-wasm-bindings.md
├── 0004-jsonld-scope.md
├── 0005-mapping-model.md
├── 0006-query-ast.md
└── 0007-reasoning-scope.md
```

---

## Testing Strategy

TripleCore needs multiple testing layers.

### Rust Unit Tests

Each crate should have focused unit tests.

```bash
cargo test
```

### Rust Integration Tests

Cross-crate tests for:

- graph serialization
- ontology loading
- mapping validation
- query planning
- diagnostics

### Python Tests

```bash
pytest
```

Test:

- import
- graph creation
- JSON-LD export
- mapping validation
- diagnostics
- OntoSQL adapter behavior

### WASM Tests

```bash
wasm-pack test --node
wasm-pack test --headless --chrome
```

Test:

- browser graph operations
- JSON-LD export
- mapping validation
- TypeScript declarations

### Snapshot Tests

Use snapshots for:

- JSON-LD output
- Turtle output
- diagnostics
- explain plans
- CLI output

### Conformance Tests

Shared fixtures:

```text
tests/fixtures/
├── people.ttl
├── organizations.ttl
├── people.mapping.yaml
├── people.query.json
├── invalid-mapping.yaml
└── expected/
```

The same fixture should run through:

```text
Rust
Python
WASM
CLI
```

---

## CI Plan

GitHub Actions should run:

```text
format
lint
rust tests
python tests
wasm tests
schema validation
docs build
examples smoke tests
```

Recommended commands:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
pytest
npm test
npm run build
wasm-pack test --node
```

---

## Release Strategy

TripleCore has three release surfaces:

```text
Crates.io
PyPI
npm
```

Use coordinated versioning at first.

Example:

```text
triplecore Rust crate:      0.1.0
triplecore Python package:  0.1.0
@triplecore/wasm:           0.1.0
@triplecore/core:           0.1.0
triplecore-cli:             0.1.0
```

Release order:

```text
1. Rust crates
2. Python wheels
3. WASM/npm packages
4. CLI binaries
5. Docs
```

Later, packages can version independently if needed.

---

## Build Tooling

Recommended developer commands in `justfile`:

```text
just fmt
just lint
just test
just test-rust
just test-python
just test-wasm
just build
just build-python
just build-wasm
just docs
just release-check
```

Example:

```just
fmt:
    cargo fmt
    ruff format python
    npm run format

lint:
    cargo clippy --all-targets --all-features -- -D warnings
    ruff check python
    npm run lint

test:
    cargo test --all-features
    pytest python/tests
    npm test
```

---

## Dependency Strategy

Start conservative.

Likely Rust dependencies:

```text
serde
serde_json
thiserror
anyhow
indexmap
smallvec
regex
url
clap
schemars
pyo3
wasm-bindgen
```

Evaluate but do not immediately depend on:

```text
oxigraph
sophia
rio
json-ld crates
```

Reason:

TripleCore should avoid becoming locked into another library's data model before your ecosystem model is stable.

---

## Public API Principles

The public API should be:

- minimal
- strongly typed
- serializable
- binding-friendly
- deterministic
- documented
- stable once released

Avoid exposing internal implementation details too early.

Use builder patterns where helpful.

Example:

```rust
let query = SemanticQuery::select("Person")
    .field("name")
    .field("email")
    .filter(eq("employer.name", "Acme"))
    .limit(10);
```

Python equivalent:

```python
query = (
    SemanticQuery.select("Person")
    .field("name")
    .field("email")
    .where("employer.name", "=", "Acme")
    .limit(10)
)
```

TypeScript equivalent:

```ts
const query = SemanticQuery
  .select("Person")
  .field("name")
  .field("email")
  .where("employer.name", "=", "Acme")
  .limit(10);
```

---

## Cursor Implementation Strategy

The monorepo should be designed so Cursor can build it incrementally.

Recommended order:

```text
1. Create workspace skeleton
2. Implement triplecore-rdf
3. Implement triplecore-diagnostics
4. Implement triplecore-jsonld
5. Implement triplecore-ontology
6. Implement triplecore-mapping
7. Implement triplecore-query
8. Implement triplecore-planner
9. Implement triplecore-cli
10. Add Python bindings
11. Add WASM bindings
12. Add docs and examples
```

Do not ask Cursor to implement the full ecosystem at once.

Give Cursor one crate and one spec at a time.

---

## MVP Definition

TripleCore MVP should include:

```text
Rust crates:
- triplecore
- triplecore-rdf
- triplecore-diagnostics
- triplecore-jsonld
- triplecore-ontology
- triplecore-mapping
- triplecore-cli

Python:
- basic triplecore package
- Graph, Iri, Literal, Triple
- JSON-LD export
- mapping validation

WASM:
- Graph
- Iri
- Literal
- JSON-LD export
- mapping validation

CLI:
- validate
- inspect
- convert
- explain
```

MVP should not include:

```text
full OWL reasoning
full SPARQL engine
full SQL execution
visual editor
database drivers
complete JSON-LD processor
```

---

## v0.1 Roadmap

### v0.1.0 — Graph Core

- RDF node model
- triples
- graph
- namespace registry
- JSON serialization
- basic tests

### v0.2.0 — Serialization

- JSON-LD writer
- N-Triples writer
- Turtle writer
- deterministic output
- snapshot tests

### v0.3.0 — Ontology Model

- ontology entities
- class/property/individual model
- labels/comments
- entity index
- inspect API

### v0.4.0 — Mapping Model

- entity mappings
- property mappings
- source paths
- identity templates
- validation diagnostics

### v0.5.0 — CLI

- validate
- inspect
- convert
- explain
- fixture-based tests

### v0.6.0 — Python Bindings

- PyO3 setup
- maturin build
- Python package
- pytest suite
- TripleModel prototype integration

### v0.7.0 — WASM Bindings

- wasm-bindgen setup
- npm package
- TypeScript declarations
- browser example
- OntoEagle prototype integration

### v0.8.0 — Query AST

- semantic query model
- filters
- projections
- paths
- query serialization

### v0.9.0 — Planner

- explain plans
- mapping-aware query planning
- path planning
- OntoSQL prototype integration

### v1.0.0 — Stable Runtime

- stable API surface
- docs complete
- Python and WASM production-ready
- ecosystem integration examples
- release automation

---

## Integration With Existing Projects

## TripleModel

TripleModel should gradually delegate graph primitives to TripleCore.

Migration path:

```text
Current TripleModel graph logic
        ↓
adapter layer
        ↓
TripleCore Graph
        ↓
Rust-backed serialization
```

Benefits:

- faster serialization
- shared RDF model
- cleaner interoperability
- stable foundation for OntoSQL/SparqlModel

---

## OntoSQL

OntoSQL should use TripleCore for:

- mapping schema
- mapping validation
- semantic query AST
- explain plans
- JSON-LD projection
- graph export

OntoSQL should keep:

- SQLModel
- SQLAlchemy
- database sessions
- Python API
- SQL execution

This avoids turning TripleCore into a database driver.

---

## SparqlModel

SparqlModel should use TripleCore for:

- RDF primitives
- ontology model
- semantic query AST
- SPARQL result mapping
- graph serialization

SparqlModel should keep:

- SPARQL endpoint execution
- graph store sessions
- Python API

---

## OntoEagle

OntoEagle should use TripleCore WASM for:

- browser-side graph model
- ontology entity model
- JSON-LD processing
- mapping visualization
- diagnostics display
- OntoSQL mapping previews

OntoEagle should keep:

- static deployment
- offline-first architecture
- search UI
- IndexedDB cache
- bundler
- CQ Ferret

---

## OntoCode

OntoCode should use TripleCore for:

- project indexing
- diagnostics
- mapping validation
- query visualization
- ontology graph previews
- LSP-style semantic feedback

---

## Ontologos

Ontologos should use TripleCore as:

- graph substrate
- ontology entity model
- inference input model
- explanation output model

Reasoning algorithms can live in Ontologos or later migrate into `triplecore-reason`.

---

## Governance

Recommended governance model:

```text
TripleCore is the stable core.
Downstream projects can move faster.
Breaking changes are carefully controlled.
Bindings are versioned with the core.
Schemas are treated as public contracts.
```

Stability tiers:

```text
Experimental
Preview
Stable
Deprecated
Removed
```

Use this in docs for every module.

---

## Success Criteria

TripleCore succeeds if:

- TripleModel can use it without losing Python ergonomics
- OntoSQL can expose mappings to OntoEagle without rewriting logic in TypeScript
- OntoEagle can run semantic logic in the browser through WASM
- OntoCode can display diagnostics from the same engine
- Ontologos can reason over the same graph model
- one fixture suite validates behavior across Rust, Python, and TypeScript
- users can inspect, validate, convert, and explain semantic artifacts from one CLI

---

## Recommended First Cursor Prompt

```text
You are building the TripleCore monorepo.

Create the initial Rust workspace and project skeleton exactly as described in TRIPLECORE_MONOREPO_PLAN.md.

Focus only on the workspace structure, root docs, crate manifests, and minimal compiling placeholder crates.

Do not implement full RDF, JSON-LD, ontology, mapping, query, planner, Python, or WASM logic yet.

Requirements:
- Root Cargo.toml with workspace members.
- crates/triplecore-rdf with placeholder public types.
- crates/triplecore-diagnostics with placeholder Diagnostic type.
- crates/triplecore umbrella crate re-exporting placeholders.
- crates/triplecore-cli with a minimal `triplecore --version` command.
- README.md with project overview.
- justfile with fmt, lint, test, build commands.
- GitHub Actions CI for cargo fmt, clippy, and tests.
- All crates compile with `cargo test`.
```

---

## Final Recommendation

Build TripleCore as a true monorepo from the beginning.

Do not make it only a Rust crate.

Do not make it only a Python extension.

Do not make it only an OntoSQL dependency.

Make it the shared runtime layer for the full semantic ecosystem.

The correct first move is a small, clean, compiling monorepo with:

```text
RDF primitives
diagnostics
serialization
ontology model
mapping model
CLI
Python bindings
WASM bindings
```

Then let TripleModel, OntoSQL, OntoEagle, OntoCode, SparqlModel, and Ontologos adopt it gradually through adapters.

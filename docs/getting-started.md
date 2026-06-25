# Getting Started

## Prerequisites

- Rust stable (1.75+)
- Optional: [just](https://github.com/casey/just) for developer commands

## Build from source

```bash
git clone https://github.com/triplecore/triplecore.git
cd triplecore
cargo test
```

## Create a graph

```rust
use triplecore::{Graph, Iri, Literal};

fn main() -> triplecore::RdfResult<()> {
    let mut graph = Graph::new();
    graph.add(
        Iri::new("https://example.com/person/1")?,
        Iri::new("https://schema.org/name")?,
        Literal::string("Ada Lovelace"),
    )?;
    println!("{}", graph.to_json_pretty()?);
    Ok(())
}
```

## CLI

```bash
cargo run -p triplecore-cli -- inspect tests/fixtures/people.json
cargo run -p triplecore-cli -- convert tests/fixtures/people.json --to jsonld
```

## Next steps

- [RDF model](rdf-model.md)
- [Architecture](../ARCHITECTURE.md)
- [Roadmap](../ROADMAP.md)

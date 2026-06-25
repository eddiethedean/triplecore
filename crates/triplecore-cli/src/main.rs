use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use triplecore::{Graph, VERSION};
use triplecore_jsonld::to_jsonld;
use triplecore_turtle::to_ntriples;

#[derive(Parser)]
#[command(name = "triplecore", version = VERSION, about = "TripleCore semantic runtime CLI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print version information
    Version,
    /// Inspect a JSON graph file
    Inspect {
        /// Path to a JSON-serialized graph
        path: PathBuf,
    },
    /// Convert between graph serialization formats
    Convert {
        /// Input JSON graph path
        input: PathBuf,
        /// Output format: json, jsonld, ntriples
        #[arg(long, default_value = "jsonld")]
        to: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        None | Some(Commands::Version) => {
            println!("triplecore {VERSION}");
        }
        Some(Commands::Inspect { path }) => {
            let graph = load_graph(&path)?;
            println!("Graph: {} triple(s)", graph.len());
            for triple in graph.iter() {
                println!("  {triple:?}");
            }
        }
        Some(Commands::Convert { input, to }) => {
            let graph = load_graph(&input)?;
            let output = match to.as_str() {
                "json" => graph.to_json_pretty()?,
                "jsonld" => to_jsonld(&graph)?,
                "ntriples" => to_ntriples(&graph)?,
                other => anyhow::bail!("unsupported output format: {other}"),
            };
            print!("{output}");
        }
    }
    Ok(())
}

fn load_graph(path: &PathBuf) -> Result<Graph> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    Graph::from_json(&contents).with_context(|| format!("failed to parse {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use triplecore::{Iri, Literal};

    #[test]
    fn example_graph_roundtrip() {
        let mut graph = Graph::new();
        graph
            .add(
                Iri::new("https://example.com/person/1").unwrap(),
                Iri::new("https://schema.org/name").unwrap(),
                Literal::string("Ada Lovelace"),
            )
            .unwrap();
        let json = graph.to_json().unwrap();
        let restored = Graph::from_json(&json).unwrap();
        assert_eq!(graph, restored);
    }
}

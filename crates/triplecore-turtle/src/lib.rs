//! Turtle and N-Triples writers (preview — full implementation planned for v0.2.0).

use triplecore_rdf::{Graph, Iri, Literal, RdfResult, Term, Triple};

/// Write a graph as N-Triples (one triple per line).
pub fn to_ntriples(graph: &Graph) -> RdfResult<String> {
    let mut lines = Vec::with_capacity(graph.len());
    for triple in graph.iter() {
        lines.push(format_triple_ntriples(triple));
    }
    Ok(lines.join("\n"))
}

fn format_triple_ntriples(triple: &Triple) -> String {
    format!(
        "{} {} {} .",
        format_term_ntriples(&triple.subject),
        format_iri_ntriples(&triple.predicate),
        format_term_ntriples(&triple.object),
    )
}

fn format_term_ntriples(term: &Term) -> String {
    match term {
        Term::Iri { iri } => format_iri_ntriples(iri),
        Term::BlankNode { node } => node.to_ntriples(),
        Term::Literal { literal } => format_literal_ntriples(literal),
    }
}

fn format_iri_ntriples(iri: &Iri) -> String {
    format!("<{}>", iri.as_str())
}

fn format_literal_ntriples(literal: &Literal) -> String {
    match literal {
        Literal::String { value } => format!("\"{}\"", escape_string(value)),
        Literal::LangString { value, language } => {
            format!("\"{}\"@{}", escape_string(value), language.as_str())
        }
        Literal::Typed { value, datatype } => {
            format!(
                "\"{}\"^^{}",
                escape_string(value),
                format_iri_ntriples(datatype.iri())
            )
        }
    }
}

fn escape_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use triplecore_rdf::Iri;

    #[test]
    fn writes_ntriples() {
        let mut graph = Graph::new();
        graph
            .add(
                Iri::new("https://example.com/person/1").unwrap(),
                Iri::new("https://schema.org/name").unwrap(),
                Literal::string("Ada Lovelace"),
            )
            .unwrap();
        let ntriples = to_ntriples(&graph).unwrap();
        assert!(ntriples.contains("<https://example.com/person/1>"));
        assert!(ntriples.ends_with(" ."));
    }
}

# TripleCore Specification (v0.1.0)

## Graph Core

### Terms

- **Iri** — validated internationalized resource identifier string
- **BlankNode** — anonymous node with local identifier
- **Literal** — plain string, language-tagged, or typed literal
- **Term** — union of IRI, blank node, or literal
- **Triple** — subject (term), predicate (IRI), object (term)
- **Graph** — insertion-ordered set of triples with merge/diff
- **NamespaceRegistry** — prefix → IRI expansion table

### JSON graph format

Graphs serialize as JSON arrays of triple objects using serde's enum tagging:

```json
[
  {
    "subject": { "kind": "iri", "iri": "https://example.com/person/1" },
    "predicate": "https://schema.org/name",
    "object": { "kind": "literal", "literal": { "kind": "string", "value": "Ada Lovelace" } }
  }
]
```

### Diagnostics

Diagnostics use stable codes (`TC-*`), severity levels, optional targets, and suggestions. See `schemas/diagnostics.schema.json`.

## Stability tiers

| Module | Tier |
|--------|------|
| `triplecore-rdf` | Preview |
| `triplecore-diagnostics` | Preview |
| Other crates | Experimental |

Preview APIs may change until v1.0.0.

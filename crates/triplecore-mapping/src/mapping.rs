use serde::{Deserialize, Serialize};
use triplecore_diagnostics::{Diagnostic, DiagnosticCollection};
use triplecore_rdf::Iri;

/// Root mapping document container.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingDocument {
    pub entities: Vec<EntityMapping>,
}

impl MappingDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> DiagnosticCollection {
        let mut diagnostics = DiagnosticCollection::new();
        for entity in &self.entities {
            if entity.name.is_empty() {
                diagnostics.push(
                    Diagnostic::error("TC-MAP-001", "Entity mapping must have a name")
                        .with_suggestion("Add a non-empty name field."),
                );
            }
            for property in &entity.properties {
                if property.source_path.is_empty() {
                    diagnostics.push(
                        Diagnostic::error(
                            "TC-MAP-002",
                            "Property mapping must reference a source path",
                        )
                        .with_target(format!("{}.{}", entity.name, property.property_iri))
                        .with_suggestion("Check the source path or schema metadata."),
                    );
                }
            }
        }
        diagnostics
    }
}

/// Maps a source record to an ontology class.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityMapping {
    pub name: String,
    pub class_iri: Iri,
    pub source: String,
    pub identity: String,
    #[serde(default)]
    pub properties: Vec<PropertyMapping>,
}

/// Maps a source field to an ontology property.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyMapping {
    pub property_iri: String,
    pub source_path: String,
    #[serde(default)]
    pub required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_empty_entity_name() {
        let doc = MappingDocument {
            entities: vec![EntityMapping {
                name: String::new(),
                class_iri: Iri::new("https://schema.org/Person").unwrap(),
                source: "people".into(),
                identity: "id".into(),
                properties: vec![],
            }],
        };
        assert!(doc.validate().has_errors());
    }
}

use serde::{Deserialize, Serialize};

/// Comparison filter for semantic queries.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: String,
    pub value: String,
}

/// Portable semantic query model.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticQuery {
    pub entity: String,
    #[serde(default)]
    pub fields: Vec<String>,
    #[serde(default)]
    pub filters: Vec<Filter>,
    #[serde(default)]
    pub limit: Option<u32>,
}

impl SemanticQuery {
    pub fn select(entity: impl Into<String>) -> Self {
        Self {
            entity: entity.into(),
            fields: Vec::new(),
            filters: Vec::new(),
            limit: None,
        }
    }

    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.fields.push(field.into());
        self
    }

    pub fn filter(mut self, field: impl Into<String>, operator: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.push(Filter {
            field: field.into(),
            operator: operator.into(),
            value: value.into(),
        });
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_query_with_builder() {
        let query = SemanticQuery::select("Person")
            .field("name")
            .filter("employer.name", "=", "Acme")
            .limit(10);
        assert_eq!(query.fields, vec!["name"]);
        assert_eq!(query.limit, Some(10));
    }
}

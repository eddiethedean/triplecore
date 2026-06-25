use serde::{Deserialize, Serialize};
use triplecore_diagnostics::DiagnosticCollection;
use triplecore_query::SemanticQuery;

/// Generated query plan.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryPlan {
    pub entity: String,
    pub steps: Vec<String>,
}

/// Explain output for a semantic query.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainPlan {
    pub query: SemanticQuery,
    pub plan: QueryPlan,
    pub diagnostics: DiagnosticCollection,
}

impl ExplainPlan {
    pub fn from_query(query: SemanticQuery) -> Self {
        let entity = query.entity.clone();
        let mut steps = vec![format!("select entity '{entity}'")];
        for field in &query.fields {
            steps.push(format!("project field '{field}'"));
        }
        for filter in &query.filters {
            steps.push(format!(
                "filter {} {} {}",
                filter.field, filter.operator, filter.value
            ));
        }
        if let Some(limit) = query.limit {
            steps.push(format!("limit {limit}"));
        }
        Self {
            query,
            plan: QueryPlan { entity, steps },
            diagnostics: DiagnosticCollection::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use triplecore_query::SemanticQuery;

    #[test]
    fn explains_simple_query() {
        let query = SemanticQuery::select("Person").field("name");
        let explain = ExplainPlan::from_query(query);
        assert_eq!(explain.plan.steps.len(), 2);
    }
}

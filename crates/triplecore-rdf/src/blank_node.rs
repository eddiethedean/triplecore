use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{RdfError, RdfResult};

/// An RDF blank node identifier.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BlankNode(String);

impl BlankNode {
    /// Create a blank node with the given local identifier (without `_:` prefix).
    pub fn new(id: impl Into<String>) -> RdfResult<Self> {
        let id = id.into();
        validate_id(&id)?;
        Ok(Self(id))
    }

    /// Return the blank node identifier (without `_:` prefix).
    pub fn id(&self) -> &str {
        &self.0
    }

    /// Format as `_:<id>`.
    pub fn to_ntriples(&self) -> String {
        format!("_:{}", self.0)
    }
}

impl fmt::Display for BlankNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_:{}", self.0)
    }
}

fn validate_id(id: &str) -> RdfResult<()> {
    if id.is_empty() {
        return Err(RdfError::InvalidBlankNode(
            "blank node id must not be empty".into(),
        ));
    }

    if id.chars().any(char::is_whitespace) {
        return Err(RdfError::InvalidBlankNode(format!(
            "blank node id must not contain whitespace: {id}"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_as_ntriples() {
        let bnode = BlankNode::new("b0").unwrap();
        assert_eq!(bnode.to_string(), "_:b0");
    }
}

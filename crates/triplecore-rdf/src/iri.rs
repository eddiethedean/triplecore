use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{RdfError, RdfResult};

/// An Internationalized Resource Identifier (IRI).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Iri(String);

impl Iri {
    /// Create a new IRI after validating the input string.
    pub fn new(value: impl Into<String>) -> RdfResult<Self> {
        let value = value.into();
        validate_iri(&value)?;
        Ok(Self(value))
    }

    /// Create an IRI without validation. Prefer [`Iri::new`] for user input.
    pub fn new_unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the IRI as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Iri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Iri {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

fn validate_iri(value: &str) -> RdfResult<()> {
    if value.is_empty() {
        return Err(RdfError::InvalidIri("IRI must not be empty".into()));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(RdfError::InvalidIri(format!(
            "IRI must not contain whitespace: {value}"
        )));
    }

    if value.starts_with('_') {
        return Err(RdfError::InvalidIri(format!(
            "IRI must not start with '_': {value}"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_iri() {
        let iri = Iri::new("https://example.com/person/1").unwrap();
        assert_eq!(iri.as_str(), "https://example.com/person/1");
    }

    #[test]
    fn rejects_empty_iri() {
        assert_eq!(
            Iri::new("").unwrap_err(),
            RdfError::InvalidIri("IRI must not be empty".into())
        );
    }

    #[test]
    fn rejects_blank_node_like_iri() {
        assert!(Iri::new("_:b0").is_err());
    }
}

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{Iri, RdfError, RdfResult};

/// An RDF datatype IRI, commonly used for typed literals.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Datatype(Iri);

impl Datatype {
    pub fn new(iri: Iri) -> Self {
        Self(iri)
    }

    pub fn iri(&self) -> &Iri {
        &self.0
    }

    pub fn xsd_string() -> RdfResult<Self> {
        Ok(Self(Iri::new("http://www.w3.org/2001/XMLSchema#string")?))
    }

    pub fn rdf_lang_string() -> RdfResult<Self> {
        Ok(Self(Iri::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString")?))
    }
}

impl fmt::Display for Datatype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A BCP47 language tag for language-tagged literals.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageTag(String);

impl LanguageTag {
    pub fn new(tag: impl Into<String>) -> RdfResult<Self> {
        let tag = tag.into();
        if tag.is_empty() {
            return Err(RdfError::InvalidLanguageTag(
                "language tag must not be empty".into(),
            ));
        }
        Ok(Self(tag))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An RDF literal value.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Literal {
    String {
        value: String,
    },
    LangString {
        value: String,
        language: LanguageTag,
    },
    Typed {
        value: String,
        datatype: Datatype,
    },
}

impl Literal {
    pub fn string(value: impl Into<String>) -> Self {
        Self::String {
            value: value.into(),
        }
    }

    pub fn lang_string(value: impl Into<String>, language: LanguageTag) -> Self {
        Self::LangString {
            value: value.into(),
            language,
        }
    }

    pub fn typed(value: impl Into<String>, datatype: Datatype) -> Self {
        Self::Typed {
            value: value.into(),
            datatype,
        }
    }

    pub fn lexical_form(&self) -> &str {
        match self {
            Self::String { value }
            | Self::LangString { value, .. }
            | Self::Typed { value, .. } => value,
        }
    }

    pub fn is_plain_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String { value } => write!(f, "\"{value}\""),
            Self::LangString { value, language } => write!(f, "\"{value}\"@{language}"),
            Self::Typed { value, datatype } => write!(f, "\"{value}\"^^{datatype}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_string_literal() {
        let lit = Literal::string("Ada Lovelace");
        assert_eq!(lit.lexical_form(), "Ada Lovelace");
        assert!(lit.is_plain_string());
    }
}

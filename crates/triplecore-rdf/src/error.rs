use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RdfError {
    #[error("invalid IRI: {0}")]
    InvalidIri(String),

    #[error("invalid blank node id: {0}")]
    InvalidBlankNode(String),

    #[error("invalid language tag: {0}")]
    InvalidLanguageTag(String),

    #[error("invalid prefix: {0}")]
    InvalidPrefix(String),
}

pub type RdfResult<T> = Result<T, RdfError>;

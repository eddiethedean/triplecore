//! Shared diagnostics: severity levels, codes, and LSP-friendly output.

mod diagnostic;
mod severity;

pub use diagnostic::{Diagnostic, DiagnosticCollection, SourceSpan};
pub use severity::Severity;

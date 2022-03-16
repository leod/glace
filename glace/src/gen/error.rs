use glsl::{parser::ParseError, syntax::IdentifierError};
use proc_macro2::Span;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenError {
    #[error("identifier error: {0:?}")]
    IdentifierError(#[from] IdentifierError),

    #[error("unsupported at {0:?}: {1}")]
    Unsupported(Span, String),
}

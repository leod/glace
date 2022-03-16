use glsl::{parser::ParseError, syntax::IdentifierError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenError {
    #[error("identifier error: {0:?}")]
    IdentifierError(#[from] IdentifierError),

    #[error("parse error: {0:?}")]
    ParseError(#[from] ParseError),
}

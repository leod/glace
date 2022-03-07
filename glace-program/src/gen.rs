use crevice::glsl::{GlslField, GlslStruct};
use glsl::{
    parser::{Parse as _, ParseError},
    syntax::{
        ArrayedIdentifier, Block, Declaration, Identifier, IdentifierError, NonEmpty,
        SingleDeclaration, StorageQualifier, StructFieldSpecifier, TypeQualifier,
        TypeQualifierSpec, TypeSpecifier,
    },
};
use thiserror::Error;

use crate::{ProgramDef, UniformInput};

#[derive(Error, Debug)]
pub enum GenError {
    #[error("identifier error: {0:?}")]
    IdentifierError(#[from] IdentifierError),

    #[error("parse error: {0:?}")]
    ParseError(#[from] ParseError),
}

pub fn uniform_block_name(identifier: &str) -> String {
    format!("__glace_uniform_block_{}", identifier)
}

pub fn uniform_block_field(field: &GlslField) -> Result<StructFieldSpecifier, GenError> {
    Ok(StructFieldSpecifier {
        qualifier: None,
        ty: TypeSpecifier::parse(field.ty)?,
        identifiers: NonEmpty::from_non_empty_iter([ArrayedIdentifier {
            ident: Identifier::new(field.name)?,
            array_spec: None,
        }])
        .unwrap(),
    })
}

pub fn uniform_block<U: GlslStruct>(identifier: &str) -> Result<Block, GenError> {
    Ok(Block {
        qualifier: TypeQualifier {
            qualifiers: NonEmpty::from_non_empty_iter([TypeQualifierSpec::Storage(
                StorageQualifier::Uniform,
            )])
            .unwrap(),
        },
        name: Identifier::new(identifier)?,
        fields: U::FIELDS
            .iter()
            .map(uniform_block_field)
            .collect::<Result<_, _>>()?,
        identifier: Some(ArrayedIdentifier {
            ident: Identifier::new(uniform_block_name(identifier))?,
            array_spec: None,
        }),
    })
}

pub fn uniform_input<U: UniformInput>() -> Vec<Declaration> {
    vec![]
}

pub fn vertex_shader<P: ProgramDef>(def: &P) {
    let source = def.vertex();
}

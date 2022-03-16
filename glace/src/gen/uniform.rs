use glsl::syntax::{
    ArrayedIdentifier, Block, Declaration, Identifier, NonEmpty, StorageQualifier,
    StructFieldSpecifier, TypeQualifier, TypeQualifierSpec,
};

use crate::{Field, UniformBlock, UniformInput};

use super::GenError;

pub fn input_name(identifier: &str) -> String {
    format!("_glace_uniform_input_{}_", identifier)
}

pub fn block_identifier(identifier: &str) -> String {
    format!("_glace_uniform_block_{}_", identifier)
}

pub fn block_field(field: &Field) -> Result<StructFieldSpecifier, GenError> {
    Ok(StructFieldSpecifier {
        qualifier: None,
        ty: field.ty.clone(),
        identifiers: NonEmpty::from_non_empty_iter([ArrayedIdentifier {
            ident: Identifier::new(field.name)?,
            array_spec: None,
        }])
        .unwrap(),
    })
}

pub fn block<U: UniformBlock>(identifier: &str) -> Result<Block, GenError> {
    Ok(Block {
        qualifier: TypeQualifier {
            qualifiers: NonEmpty::from_non_empty_iter([TypeQualifierSpec::Storage(
                StorageQualifier::Uniform,
            )])
            .unwrap(),
        },
        name: Identifier::new(block_identifier(identifier))?,
        fields: U::FIELDS
            .iter()
            .map(block_field)
            .collect::<Result<_, _>>()?,
        identifier: Some(ArrayedIdentifier {
            ident: Identifier::new(input_name(identifier))?,
            array_spec: None,
        }),
    })
}

pub fn input<U: UniformInput>() -> Vec<Declaration> {
    vec![]
}

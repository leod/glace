use crevice::{glsl::GlslStruct, std140::AsStd140};
use glsl::syntax::TypeSpecifier;

pub trait NamedInput {
    const NAMES: &'static [&'static str];
}

pub trait ConstInput: NamedInput {}

pub trait Uniform: AsStd140 + GlslStruct {}

pub trait UniformInput: NamedInput {}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub offset: usize,
    pub type_spec: TypeSpecifier,
}

pub trait Vertex: NamedInput {
    const ATTRIBUTES: &'static [Attribute];
}

use crevice::{glsl::GlslStruct, std140::AsStd140};
use glsl::syntax::TypeSpecifier;

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub offset: usize,
    pub type_spec: TypeSpecifier,
}

pub trait Vertex {
    const ATTRIBUTES: &'static [Attribute];
}

pub trait Uniform: AsStd140 + GlslStruct {}

pub trait UniformInputs {
    type Bindings: UniformBindings;

    const NAMES: &'static [&'static str];
}

pub trait UniformBindings {
    type Inputs: UniformInputs;
}

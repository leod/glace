use crevice::{glsl::GlslStruct, std140::AsStd140};
use glsl::syntax::{Expr, TypeSpecifier};

pub trait NamedFields {
    const FIELD_NAMES: &'static [&'static str];
}

pub trait ConstInput: NamedFields {
    fn const_values(&self) -> Vec<Expr>;
}

pub trait Uniform: AsStd140 + GlslStruct {}

pub trait UniformInput: NamedFields {}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub offset: usize,
    pub type_spec: TypeSpecifier,
}

pub trait Vertex: NamedFields {
    const ATTRIBUTES: &'static [Attribute];
}

pub trait VertexInput: NamedFields {}

pub trait VertexOutput: NamedFields {}

pub trait FragmentOutput: NamedFields {}

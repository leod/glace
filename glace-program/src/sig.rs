use crevice::{glsl::GlslStruct, std140::AsStd140};
use glsl::syntax::{Expr, TypeSpecifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub type_specifier: TypeSpecifier,
}

pub trait Fields {
    const FIELDS: &'static [Field];
}

pub trait ConstInput: Fields {
    fn const_values(&self) -> Vec<Expr>;
}

pub trait Uniform: AsStd140 + GlslStruct {}

pub trait UniformInput: Fields {}

pub trait Vertex: Fields {
    const OFFSETS: &'static [usize];
}

pub trait VertexInput: Fields {}

pub trait VertexOutput: Fields {}

pub trait FragmentOutput: Fields {}

use crevice::{glsl::GlslStruct, std140::AsStd140};
use glsl::syntax::{Declaration, Expr, Statement, TypeSpecifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub type_specifier: TypeSpecifier,
}

pub trait Fields {
    const FIELDS: &'static [Field];
}

pub trait ConstInput {
    fn const_expr(&self) -> Expr;
}

pub trait UniformValue {}

pub trait UniformBlock: AsStd140 + GlslStruct {
    const FIELDS: &'static [Field];
}

pub trait UniformInput: Fields {
    fn declarations() -> Vec<Declaration>;
}

pub trait Vertex: Fields {
    const OFFSETS: &'static [usize];
}

pub trait VertexInput: Fields {}

pub trait VertexOutput: Fields {}

pub trait FragmentOutput: Fields {}

pub struct VertexShaderDef {}

pub trait ProgramDef {
    type UniformInput: UniformInput;
    type VertexInput: VertexInput;
    type VertexOutput: VertexOutput;
    type FragmentOutput: FragmentOutput;

    fn vertex(&self) -> Statement;
    fn fragment(&self) -> Statement;
}

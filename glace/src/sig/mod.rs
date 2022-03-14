mod uniform_type;

use crevice::std140::AsStd140;
use glsl::syntax::{Declaration, Expr, Statement, TypeSpecifier};

pub use uniform_type::UniformType;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: &'static str,
    pub ty: TypeSpecifier,
}

pub trait Fields {
    const FIELDS: &'static [Field];
}

pub trait ConstValue {
    fn const_expr(&self) -> Expr;
}

pub unsafe trait UniformBlock: AsStd140 {
    const NAME: &'static str;
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

pub trait ProgramDef {
    type Uniform: UniformInput;
    type Vertex: VertexInput;
    type Varying: VertexOutput;
    type Fragment: FragmentOutput;

    fn vertex(&self) -> Statement;
    fn fragment(&self) -> Statement;

    #[doc(hidden)]
    fn _glace_type_check_vertex(
        &self,
        _uniform: &Self::Uniform,
        _vertex: &Self::Vertex,
        _varying: &mut Self::Varying,
    ) {
        unreachable!("`ProgramDef::_glace_type_check_vertex` should never be called.");
    }

    #[doc(hidden)]
    fn _glace_type_check_fragment(
        &self,
        _uniform: &Self::Uniform,
        _varying: &Self::Varying,
        _fragment: &mut Self::Fragment,
    ) {
        unreachable!("`ProgramDef::_glace_type_check_fragment` should never be called.");
    }
}

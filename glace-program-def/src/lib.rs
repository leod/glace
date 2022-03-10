pub use crevice;
pub use glsl;

mod sig;

pub mod gen;

pub use sig::{
    ConstInput, Fields, ProgramDef, UniformBlock, UniformInput, UniformValue, VertexInput,
    VertexOutput,
};

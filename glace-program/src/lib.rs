pub use crevice;
pub use glsl;

mod gen;
mod sig;

pub use sig::{
    ConstInput, Fields, ProgramDef, UniformBlock, UniformInput, VertexInput, VertexOutput,
};

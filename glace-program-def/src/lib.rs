pub use crevice;
pub use glsl;

mod sig;

pub mod gen;

pub use sig::{
    ConstInput, Field, ProgramDef, UniformBlock, UniformInput, UniformType, VertexInput,
    VertexOutput,
};

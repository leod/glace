pub use crevice;
pub use glsl;

mod sig;

pub mod gen;

pub use sig::{
    ConstValue, Field, ProgramDef, UniformBlock, UniformInput, UniformType, VertexInput,
    VertexOutput,
};

#[cfg(feature = "derive")]
pub use glace_derive::UniformBlock;

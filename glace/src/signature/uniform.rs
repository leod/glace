use crevice::{glsl::GlslStruct, std140::AsStd140};

pub trait Uniform: AsStd140 + GlslStruct {}

pub trait UniformInput {
    type Bindings: UniformBindings;
}

pub trait UniformBindings {
    type Input: UniformInput;
}

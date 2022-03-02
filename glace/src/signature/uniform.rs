use crevice::{glsl::GlslStruct, std140::AsStd140};

pub trait Uniform: AsStd140 + GlslStruct {}

pub trait UniformInputs {
    type Bindings: UniformBindings;

    const NAMES: &'static [&'static str];
}

pub trait UniformBindings {
    type Inputs: UniformInputs;
}

use std::marker::PhantomData;

use crate::sig::{ConstInput, Field, FragmentOutput, UniformInput, VertexInput, VertexOutput};

pub struct VertexShaderDef<
    Consts: ConstInput,
    Uniforms: UniformInput,
    Vertex: VertexInput,
    Varyings: VertexOutput,
> {
    source: String,

    _phantom: PhantomData<(Consts, Uniforms, Vertex, Varyings)>,
}

pub struct FragmentShaderDef<
    Consts: ConstInput,
    Uniforms: UniformInput,
    Varyings: VertexOutput,
    Fragment: FragmentOutput,
> {
    source: String,

    _phantom: PhantomData<(Consts, Uniforms, Varyings, Fragment)>,
}

pub struct ProgramDef<
    Consts: ConstInput,
    Uniforms: UniformInput,
    Vertex: VertexInput,
    Fragment: FragmentOutput,
> {
    varyings: Vec<Field>,
    vertex_shader_source: String,
    fragment_shader_source: String,

    _phantom: PhantomData<(Consts, Uniforms, Vertex, Fragment)>,
}

impl<Consts: ConstInput, Uniforms: UniformInput, Vertex: VertexInput, Fragment: FragmentOutput>
    ProgramDef<Consts, Uniforms, Vertex, Fragment>
{
    pub fn new<Varyings: VertexOutput>(
        vertex_shader: impl FnOnce() -> VertexShaderDef<Consts, Uniforms, Vertex, Varyings>,
        fragment_shader: impl FnOnce() -> FragmentShaderDef<Consts, Uniforms, Varyings, Fragment>,
    ) -> Self {
        let varyings = Vec::new(); // TODO

        Self {
            varyings,
            vertex_shader_source: vertex_shader().source,
            fragment_shader_source: fragment_shader().source,
            _phantom: PhantomData,
        }
    }
}

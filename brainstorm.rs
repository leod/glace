#[derive(glace::UniformBlock)]
struct ViewMatrices {
    world_to_camera: Matrix4<f32>,
    camera_to_ndc: Matrix4<f32>,
}

#[derive(glace::Vertex)]
struct Vertex {
    tex: Vector2<f32>,
    pos: Vector3<f32>,
}

#[derive(glace::Vertex)]
struct Instance {
    color: Vector3<f32>,
}

#[derive(glace::UniformSet)]
struct UniformSet {
    #[layout(binding = 0)]
    view_matrices: UniformData<ViewMatrices>,

    #[layout(binding = 1)]
    texture: UniformSampler<Texture2d>,
}

#[derive(glace::VertexSet)]
struct VertexSet {
    instance: VertexData<Instance>,
    vertex: VertexData<Vertex>,
}

#[derive(glace::VaryingFields)]
struct VaryingFields {
    color: Vector3<f32>,
}

#[derive(glace::FragmentFields)]
struct FragmentFields {
    albedo: Vector3<f32>,
    normal: Vector3<f32>,
}

struct MyProgram {
    radians_per_sec: f32,
}

impl ProgramDef for MyProgram {
    type UniformSet = UniformSet;
    type VertexSet = VertexSet;
    type VaryingFields = VaryingFields;
    type FragmentFields = FragmentFields;

    #[glace(vertex_shader)]
    fn vertex_shader(
        &self,
        uniform: UniformSet,
        input: VertexSet,
        output: &mut VaryingFields,
    ) {
        gl_Position = uniform.view_matrices.camera_to_ndc
            * uniform.view_matrices.world_to_camera
            * vec4(input.vertex.position, 1.0);

        output.color = vertex.instance.color * texture(uniform.texture, input.vertex.tex);
    }

    #[glace(fragment_shader)]
    fn fragment_shader(
        &self,
        uniform: UniformSet,
        input: VaryingFields,
        output: &mut FragmentFields,
    ) {
        output.albedo = input.color;
        output.normal = vec3(1.0, 0.0, 0.0);
    }
}

struct Renderer {
    program: Program<UniformSet, VertexSet, FragmentFields>,
    view_matrices: UniformBuffer<ViewMatrices>,
    texture: Texture2d,
}

impl Renderer {
    pub fn new(glace: &glace::Context) -> Result<Self, glace::InitError> {
        let program_def = MyProgram {
            radians_per_sec: 3.0,
        };
        let program = Program::new(glace, program_def)?;
        let view_matrices = UniformBuffer::new(glace, ViewMatrices::default())?;
        let texture = Texture::new(glace)?;

        Ok(Self {
            program,
            view_matrices,
            texture,
        })
    }

    pub fn draw(
        &self,
        data: &VertexArray<VertexSet>,
    ) {
        self.program.draw(
            UniformSet {
                view_matrices: self.view_matrices.data(),
                texture: self.texture.sampler(),
            },
            data,
            &draw_params,
            &framebuffer,
        );
    }
}

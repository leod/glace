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
    col: Vector3<f32>,
}

#[derive(glace::UniformSet)]
struct UniformSet {
    #[layout(binding = 0)]
    view_matrices: UniformBlockInput<ViewMatrices>,

    #[layout(binding = 1)]
    texture: UniformSamplerInput<Texture2d>,
}

#[derive(glace::VertexSet)]
struct VertexSet {
    instance: VertexInput<Instance>,

    #[layout(divisor = 1)]
    vertex: VertexInput<Vertex>,
}

#[derive(glace::Varying)]
struct Varying {
    col: Vector3<f32>,
}

#[derive(glace::FragmentSet)]
struct FragmentSet {
    albedo: FragmentOutput<Vector3<f32>>,
    normal: FragmentOutput<Vector3<f32>>,
}

struct MyProgram {
    radians_per_sec: f32,
}

impl ProgramDef for MyProgram {
    type UniformSet = UniformSet;
    type VertexSet = VertexSet;
    type Varying = Varying;
    type FragmentSet = FragmentSet;

    #[glace(vertex_shader)]
    fn vertex_shader(
        &self,
        uniform: UniformSet,
        input: VertexSet,
        output: &mut Varying,
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
        output: &mut Varying,
    ) {
        output.albedo = input.color;
        output.normal = vec3(1.0, 0.0, 0.0);
    }
}

struct Renderer {
    program: Program<UniformSet, VertexSet, FragmentSet>,
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
        vertices: &VertexStream<VertexSet>,
        framebuffer: &Framebuffer<FragmentSet>,
    ) {
        self.program.draw(
            UniformSet {
                view_matrices: self.view_matrices.input(),
                texture: self.texture.input(),
            },
            vertices,
            &framebuffer,
            &draw_params,
        );
    }
}

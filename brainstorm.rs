
// ----------------------------------

#[derive(glace::Define)]
struct Define {
    radians_per_sec: f32,
}

#[derive(glace::Uniform)]
struct ViewMatrices {
    world_to_camera: Matrix4<f32>,
    camera_to_ndc: Matrix4<f32>,
}

#[derive(glace::Uniform)]
#[glace(bindings = UniformBindings)]
struct Uniform {
    view_matrices: ViewMatrices,
    texture: Texture2d,
}

#[derive(glace::Vertex)]
struct Vertex {
    position: Vector3<f32>,
    uv: Vector2<f32>,
}

#[derive(glace::Varying)]
struct Varying {
    color: Vector3<f32>,
}

#[derive(glace::Fragment)]
struct Fragment {
    albedo: Vector3<f32>,
    normal: Vector3<f32>,
}

#[glace(vertex)]
fn vertex(
    define: &Define,
    uniform: &Uniform,
    vertex: &Vertex,
    varying: &mut Varying,
) {
    vec3 color = texture(uniform.texture, vertex.uv);
    varying.position = uniform.view_matrices.camera_to_ndc
        * uniform.view_matrices.world_to_camera
        * vec4(vertex.position, 1.0);
}

#[glace(fragment)]
fn fragment(
    define: &Define,
    uniform: &Uniform,
    varying: &Varying,
    fragment: &mut Fragment,
) {
    fragment.albedo = varying.color;
    fragment.normal = vec3(1.0, 0.0, 0.0);
}

struct Renderer {
    program: Program<Uniform, Vertex, Fragment>,
    view_matrices: UniformBuffer<ViewMatrices>,
    texture: Texture2d,
}

impl Renderer {
    pub fn new(glace: &glace::Context) -> Result<Self, glace::InitError> {
        let define = Define {
            radians_per_sec: 1.0,
        };
        let program = Program::new(glace, vertex, fragment, define)?;
        let view_matrices = UniformBuffer::new(glace, ViewMatrices::default())?;
        let texture = Texture::load("bla.png")?;

        Ok(Self {
            program,
            view_matrices,
            texture,
        })
    }

    pub fn draw(
        &self,
        data: &VertexData<Vertex>,
    ) {
        self.program.draw(
            &UniformBindings {
                view_matrices: &self.view_matrices,
                texture: &self.texture,
            },
            data,
        );
    }
}
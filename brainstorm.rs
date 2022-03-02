
// ----------------------------------

#[derive(Uniform)]
struct ViewMatrices {
    world_to_camera: Matrix4<f32>,
    camera_to_ndc: Matrix4<f32>,
}

#[derive(Vertex)]
struct TexVertex {
    position: Vector3<f32>,
    uv: Vector2<f32>,
}

#[derive(ConstInput)]
struct Consts {
    radians_per_sec: f32,
}

#[derive(UniformInput)]
#[glace(bindings = UniformBindings)]
struct Uniforms {
    view_matrices: ViewMatrices,
    texture: Texture2d,
}

#[derive(VertexOutput)]
struct Varyings {
    color: Vector3<f32>,
}

#[derive(FragmentOutput)]
struct Fragment {
    albedo: Vector3<f32>,
    normal: Vector3<f32>,
}

#[glace]
fn foo(
    color: Vector3<f32>,
) {
    
}

fn foo(consts: &Consts) -> String {

}

#[glace(vertex, requires(foo))]
fn vertex(
    uniforms: &Uniforms,
    vertex: &Vertex,
    varyings: &mut Varyings,
) {
    varyings.sdfd = foo(vertex.color);
    varyings.position = uniforms.view_matrices.camera_to_ndc
        * uniforms.view_matrices.world_to_camera
        * vec4(vertex.position, 1.0);

    varyings.color = texture(uniforms.texture, vertex.uv);
}

#[glace(fragment)]
fn fragment(
    uniforms: &Uniforms,
    varyings: &Varyings,
    fragment: &mut Fragment,
) {
    fragment.albedo = varyings.color;
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
        let texture = Texture::new(glace)?;

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
            &draw_params,
            &framebuffer,
        );
    }
}
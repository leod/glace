use glsl::syntax::TypeSpecifier;

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub offset: usize,
    pub type_spec: TypeSpecifier,
}

pub trait Vertex {
    const ATTRIBUTES: &'static [Attribute];
}

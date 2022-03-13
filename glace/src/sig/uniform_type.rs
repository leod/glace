use glsl::syntax::{TypeSpecifier, TypeSpecifierNonArray};

pub trait UniformType {
    const TYPE_SPECIFIER: TypeSpecifier;
}

impl UniformType for f32 {
    const TYPE_SPECIFIER: TypeSpecifier = TypeSpecifier {
        ty: TypeSpecifierNonArray::Float,
        array_specifier: None,
    };
}

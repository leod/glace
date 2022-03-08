use crevice::{glsl::GlslStruct, std140::AsStd140};
use glsl::{
    syntax::Declaration,
    transpiler::glsl::{show_block, show_declaration},
};

#[derive(GlslStruct, AsStd140)]
struct CameraMatrices {
    view: f32,
    projection: f32,
}

fn main() {
    let tree = glace_program::gen::uniform_block::<CameraMatrices>("camera_matrices").unwrap();

    let mut s = String::new();
    show_block(&mut s, &tree);
    println!("{}", s);

    let mut s = String::new();
    show_declaration(&mut s, &Declaration::Block(tree));
    println!("{}", s);
}

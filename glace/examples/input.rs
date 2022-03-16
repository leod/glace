use crevice::std140::AsStd140;
use glace::UniformBlock;
use glsl::{
    syntax::Declaration,
    transpiler::glsl::{show_block, show_declaration},
};

#[derive(AsStd140, UniformBlock)]
struct CameraMatrices {
    view: f32,
    projection: f32,
}

fn main() {
    let tree = glace::gen::uniform::block::<CameraMatrices>("camera_matrices").unwrap();

    let mut s = String::new();
    show_block(&mut s, &tree);
    println!("{}", s);

    let mut s = String::new();
    show_declaration(&mut s, &Declaration::Block(tree));
    println!("{}", s);
}

use crate::ProgramDef;

fn gen_vertex_shader<P: ProgramDef>(def: &P) {
    let source = def.vertex();
}

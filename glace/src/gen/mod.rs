mod error;

pub mod uniform;

pub use error::GenError;

use crate::ProgramDef;

pub fn vertex_shader<P: ProgramDef>(def: &P) {
    let source = def.vertex();
}

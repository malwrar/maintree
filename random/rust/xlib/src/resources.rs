use crate::render::mesh::{Mesh, MeshBuilder};
use crate::render::shader::ShaderProgram;

#[derive(Debug, PartialEq)]
pub struct WindowRenderResource {
    pub shader: ShaderProgram,
    pub mesh: Mesh,
}
        
impl WindowRenderResource {
    pub fn new() -> Self {
        let shader = ShaderProgram::new_basic(
            include_str!("../assets/shaders/basic.vert"),
            include_str!("../assets/shaders/basic.frag")).unwrap();
        
	    let mesh = MeshBuilder::new()
            .vertex_data(&[
               -0.5,  0.5, 0.0, 0.0, 1.0,  // upper left
                0.5,  0.5, 0.0, 1.0, 1.0,  // upper right
               -0.5, -0.5, 0.0, 0.0, 0.0,  // lower left
                0.5, -0.5, 0.0, 1.0, 0.0,  // lower right
            ])
            .indices(&[
                0, 1, 2,
                1, 2, 3,
            ])
            .attribute(0, 3)
            .attribute(1, 2)
            .build();

        Self {
            shader,
            mesh,
        }
    }
}

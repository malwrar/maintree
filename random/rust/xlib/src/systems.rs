use std::time::Instant;

use legion::*;

use log::debug;

use crate::{
    components::{Position, Orientation},
    resources::WindowRenderResource,
    compositor::{
        ManagedWindow,
    },
    window_system::{
        Window,
    },
    render::camera::Camera,
};

#[system(for_each)]
pub fn draw_window(
    #[resource] time: &Instant,
    #[resource] camera: &mut Camera,
    #[resource] rendering: &WindowRenderResource,
    position: &Position,
    angles: &Orientation,
    window: &ManagedWindow,
) {
    // Set up transform matrix.
    camera.look_at(position.vec);
    let rotation = ((time.elapsed().as_millis() as f32 * 360.0) / 2000.0) % 360.0;
    
    let attrs = window.get_attributes().unwrap();
    let model_scale = glm::normalize(glm::vec3(
            1.0,
            attrs.height as f32 / attrs.width as f32,
            1.0));
    
    let modelview = camera.calc_view_matrix();
    let modelview = glm::ext::translate(&modelview, position.vec);
    let modelview = glm::ext::scale(&modelview, model_scale);
    let modelview = glm::ext::rotate(&modelview,
            glm::radians(rotation), glm::vec3(0.0, 1.0, 0.0));
    let projection = camera.calc_projection_matrix();
    
    let texture = window.get_texture().unwrap();
    texture.activate();

    rendering.shader.activate();
    
    rendering.shader.set_uniform_mat4f(
           rendering.shader.get_uniform("modelview").unwrap(),
           &modelview);
    rendering.shader.set_uniform_mat4f(
           rendering.shader.get_uniform("projection").unwrap(),
           &projection);

    rendering.mesh.draw_triangles();
}

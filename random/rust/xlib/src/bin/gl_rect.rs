use std::rc::Rc;

use xlib_sandbox::{
    display::Display,
    extension::{
        gl::GlWindow,
        RenderTarget,
    },
    render::{
        camera::Camera,
        mesh::MeshBuilder,
        shader::ShaderProgram,
    },
    window::Window,
};

use glm;

fn main() {
    env_logger::init();

    // Set up x window stuff
    let display = Display::local().unwrap();
    let root_window = display.root_window().unwrap();

    let gl_window = GlWindow::create(&root_window, 4, 0).unwrap();
    gl_window.set_size(1280, 720).unwrap();

    // Set up common render resources
    let root_window_attrs = root_window.get_attributes().unwrap();
    let mut camera = Camera::new(glm::vec3(0.0, 0.0, 0.0),
        root_window_attrs.width, root_window_attrs.height);

    let shader = ShaderProgram::new_basic(
        include_str!("gl_rect.vert"),
        include_str!("gl_rect.frag")).unwrap();

    let mesh = MeshBuilder::new()
        .vertex_data(&[
            -0.5,  0.5, 0.0,  // upper left
             0.5,  0.5, 0.0,  // upper right
            -0.5, -0.5, 0.0,  // lower left
             0.5, -0.5, 0.0,  // lower right
        ])
        .indices(&[
            0, 1, 2,
            1, 2, 3,
        ])
        .attribute(0, 3)
        .build();


    // Render scene
    let object_pos = glm::vec3(0.0, 0.0, -1.0);

    loop {
        gl_window.start_frame().unwrap();
        shader.activate();

        camera.look_at(object_pos);

        let aspect_ratio = root_window_attrs.height as f32
            / root_window_attrs.height as f32;

        let model_scale = glm::normalize(glm::vec3(1.0, 1.0 * aspect_ratio, 1.0));

        let modelview = camera.calc_view_matrix();
        let modelview = glm::ext::translate(&modelview, object_pos);
        let modelview = glm::ext::scale(&modelview, model_scale);
        let projection = camera.calc_projection_matrix();

        shader.set_uniform_mat4f(
                shader.get_uniform("modelview").unwrap(),
                &modelview);
        shader.set_uniform_mat4f(
                shader.get_uniform("projection").unwrap(),
                &projection);

        mesh.draw_triangles();

        gl_window.render_frame().unwrap();
    }
}
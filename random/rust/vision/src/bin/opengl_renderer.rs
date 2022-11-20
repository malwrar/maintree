//!

use opencv::{
	core::{
		Point3_,
	},
	highgui,
	prelude::*,
	Result,
	videoio,
};

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
		texture::Texture,
    },
    window::{
		Window,
		WindowRef,
	},
};

use glm;

fn main() -> Result<()> {
	// Init CV stuff
    let mut file = videoio::VideoCapture::from_file("./assets/office_calib_iphone/orbit_left_right.mov", videoio::CAP_ANY)?;

	if !videoio::VideoCapture::is_opened(&file)? {
		panic!("Unable to open file!");
	}

    // Set up x stuff
    let display = Display::local().unwrap();
    let root_window = display.root_window().unwrap();
	let display_window = WindowRef::create_simple(&root_window, 0, 0, 1280, 720, 0, 0, 0).unwrap();
	display_window.show().unwrap();

    let gl_window = GlWindow::create(&display_window, 4, 0).unwrap();
    gl_window.set_size(1280, 720).unwrap();

    // Set up common render resources
    let root_window_attrs = root_window.get_attributes().unwrap();
    let mut camera = Camera::new(glm::vec3(0.0, 0.0, 0.0),
        root_window_attrs.width, root_window_attrs.height);

    let background_shader = ShaderProgram::new_basic(
        include_str!("background.vert"),
        include_str!("background.frag")).unwrap();

    let background_mesh = MeshBuilder::new()
        .vertex_data(&[
            -1.0,  1.0, 0.0, 0.0, 1.0,  // upper left
             1.0,  1.0, 0.0, 1.0, 1.0,  // upper right
            -1.0, -1.0, 0.0, 0.0, 0.0,  // lower left
             1.0, -1.0, 0.0, 1.0, 0.0,  // lower right
        ])
        .indices(&[
            0, 1, 2,
            1, 2, 3,
        ])
        .attribute(0, 3)
        .attribute(1, 2)
        .build();

	loop {
		if highgui::wait_key(10)? > 0 { break; }

		let mut frame = Mat::default();
		if !file.read(&mut frame).expect("Failed to read next frame.") {
            break;
        }

		// Copy frame to texture
		let width = frame.cols();
		let height = frame.rows();

		let mut pixels = Vec::new();
		for y in 0..height {
			for x in 0..width {
				let pixel: Point3_<u8> = *frame.at_2d(y, x).unwrap();
				pixels.push(pixel.z);  // b
				pixels.push(pixel.y);  // g
				pixels.push(pixel.x);  // r
				pixels.push(255);      // a
			}
		}

        let background_texture = Texture::new()
            .from_data(width, height, gl::RGBA, pixels)
            .set_param(gl::TEXTURE_WRAP_S, gl::REPEAT)
            .set_param(gl::TEXTURE_WRAP_T, gl::REPEAT)
            .set_param(gl::TEXTURE_MAG_FILTER, gl::LINEAR)
            .set_param(gl::TEXTURE_MIN_FILTER, gl::LINEAR)
			.flip_horizontally()
            .build();

		// Render
        gl_window.start_frame().unwrap();

        background_texture.activate();
        background_shader.activate();

        background_mesh.draw_triangles();

        gl_window.render_frame().unwrap();
	}

	Ok(())
}
use winit::event::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

/// Structure containing stuff needed to do the usual world-to-screen transform math in a
/// shader-compatible format. Example WGSL definition below demonstrating usage:
///
///   [[block]] // 1.
///   struct CameraUniform {
///       view_proj: mat4x4<f32>;
///   };
///   [[group(1), binding(0)]] // 2.
///   var<uniform> camera: CameraUniform;
///
/// This uniform can be updated using a Perspective object
#[repr(C)]  // We need this for Rust to store our data correctly for the shaders
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn from(perspective: &Perspective) -> Self {
        let mut uniform = Self::new();

        let view = cgmath::Matrix4::look_at_rh(perspective.eye,
                perspective.target, perspective.up);

        let proj = cgmath::perspective(cgmath::Deg(perspective.fovy),
                perspective.aspect, perspective.znear, perspective.zfar);

        uniform.view_proj = (OPENGL_TO_WGPU_MATRIX * proj * view).into();

        uniform
    }
}

pub struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::LControl => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_perspective(&self, perspective: &mut Perspective) {
        use cgmath::InnerSpace;
        let forward = perspective.target - perspective.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            perspective.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            perspective.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(perspective.up);

        let forward = perspective.target - perspective.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            perspective.eye = perspective.target
                - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            perspective.eye = perspective.target
                - (forward - right * self.speed).normalize() * forward_mag;
        }

        if self.is_up_pressed {
        }
        if self.is_down_pressed {
        }

    }
}


/// Represents a perspective in some 3d cartesian space.
pub struct Perspective {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Perspective {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: width / height,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn get_camera_uniform(&self) -> CameraUniform {
        CameraUniform::from(&self)
    }
}


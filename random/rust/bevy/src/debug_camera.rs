use bevy::{
    prelude::*,
    app::Plugin,
    input::mouse::{
        MouseButton,
        MouseMotion,
        MouseWheel,
    },
};

#[derive(Default)]
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(tilt_and_zoom_camera);
    }
}

#[derive(Component)]
pub struct CameraController {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

fn tilt_and_zoom_camera(
    mut windows: ResMut<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    button: Res<Input<MouseButton>>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
) {
    let window = windows.get_primary_mut().unwrap();

    // Make the mouse invisible and locked to the center of the screen when
    // the tilt button is held.
    if button.just_pressed(MouseButton::Right) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    } else if button.just_released(MouseButton::Right) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }

    // If the button isn't held at this point, we don't need to continue on to
    // altering the camera's rotation.
    if !button.pressed(MouseButton::Right) { return; }

    // Calculate how far our mouse cursor and scroll wheel has moved.
    let mut cursor_delta = Vec2::ZERO;
    for ev in ev_motion.iter() {
        cursor_delta += ev.delta;
    }

    let mut scroll_delta = 0.0;
    for ev in ev_scroll.iter() {
        scroll_delta += ev.y;
    }

    // Alter the rotation of the debug camera.
    for (_camera, mut transform) in query.iter_mut() {
        let delta = cursor_delta / Vec2::new(window.width(), window.height());
        let pitch = Quat::from_rotation_x(-delta.y);
        let yaw = Quat::from_rotation_y(-delta.x);

        transform.rotation = (yaw * transform.rotation) * pitch;
    }
}
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
            .add_system(control_camera);
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

fn spawn_camera(
    mut windows: ResMut<Windows>,
    mut commands: Commands,
) {
    let window = windows.get_primary_mut().unwrap();

    // We assume the cursor is visible and freely manipulable.
    window.set_cursor_lock_mode(false);
    window.set_cursor_visibility(true);

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(CameraController::default());
}

fn control_camera(
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
    button: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
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

    if let Ok((_camera, mut transform)) = query.get_single_mut() {
        // Translate the camera.
        let speed = 0.1;
        if key.pressed(KeyCode::W) {
            transform.translation.z += -speed;
        }
        if key.pressed(KeyCode::S) {
            transform.translation.z += speed;
        }
        if key.pressed(KeyCode::A) {
            transform.translation.x += -speed;
        }
        if key.pressed(KeyCode::D) {
            transform.translation.x += speed;
        }

        // Calculate how far our mouse cursor and scroll wheel has moved.
        let mut cursor_delta = Vec2::ZERO;
        let mut scroll_delta = 0.0;

        if button.pressed(MouseButton::Right) {
            for ev in ev_motion.iter() {
                cursor_delta += ev.delta;
            }
            println!("cursor_delta: {}", cursor_delta);

            for ev in ev_scroll.iter() {
                scroll_delta += ev.y;
            }

            println!("scroll_delta: {}", scroll_delta);
        }

        // Rotate the camrea
        let delta = cursor_delta / Vec2::new(window.width(), window.height());
        let pitch = Quat::from_rotation_x(-delta.y);
        let yaw = Quat::from_rotation_y(-delta.x);

        transform.rotation = (yaw * transform.rotation) * pitch;
    }
}

fn translate_camera(
    mut windows: ResMut<Windows>,
    key: Res<Input<KeyCode>>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
) {
    // Alter the rotation of the debug camera.
    for (_camera, mut transform) in query.iter_mut() {

    }
}
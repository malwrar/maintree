use bevy::prelude::*;

use bevy_sandbox::debug_camera::{CameraController, CameraControllerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraControllerPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 10.0),
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 3.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..default()
    })
    .insert(CameraController::default());

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let scale = 2.0;
    let root_width = 1920.0;
    let root_height = 1080.0;
    let aspect_ratio = root_height / root_width;

    let fake_windows = [
        ((0.0, 0.0), (root_width, root_height), 0.0, Color::rgb(1.0, 0.0, 0.0)),
        ((0.5 * root_width, 0.0), (600.0, 400.0), 0.1, Color::rgb(0.0, 0.0, 1.0)),
        ((0.0, 0.5 * root_height), (400.0, 200.0), 0.2, Color::rgb(0.0, 1.0, 0.0)),
    ];

    for ((x, y), (width, height), z, color) in fake_windows {
        let width =  (width / root_width);
        let height = (height / root_height) * aspect_ratio;

        let x = ((x / root_width) + (0.5 * width) - 0.5);
        let y = (-((y / root_height) * aspect_ratio) + (0.5 * (1.0 - height)) - 0.25);

        let transform = Transform::from_xyz(scale * x, scale * y + 2.0, z)
            .with_rotation(Quat::from_rotation_x(0.0));

        let entity = commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad { 
                size: scale * Vec2::new(width, height),
                flip: false,
            })),
            material: materials.add(color.into()),
            transform,
            ..default()
        });
    }

}
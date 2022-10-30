//! Basic demo of bevy capabilities.

use std::fs;
use std::io;
use std::path::PathBuf;

use bevy::{
    /*
    render::{
        mesh::Indices,
        texture::{Image, ImageType},
        render_resource::PrimitiveTopology,
    },
    */
    prelude::*,
};

use bevy_infinite_grid::GridShadowCamera;

use bevy_sandbox::{
    debug::DebugPlugin,
    debug_camera::{
        CameraController,
        CameraControllerPlugin
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(hello_world)
        .add_startup_system(setup_scene)
        .run();
}

fn hello_world() {
    println!("Hello world.");
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 3.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..default()
    })
    .insert(CameraController::default())
    .insert(GridShadowCamera);

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
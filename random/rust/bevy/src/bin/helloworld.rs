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

use bevy_sandbox::debug_camera::DebugCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugCameraPlugin::default())
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
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
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
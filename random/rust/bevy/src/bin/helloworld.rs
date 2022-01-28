use std::fs;
use std::path::PathBuf;

use bevy::{
    render::texture::{Image, ImageType},
    prelude::*,
};

use malicious::kitti;
use malicious::bevy::CameraPlugin;

fn hello_world() {
    println!("Hello world.");
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin {})
        .add_startup_system(hello_world)
        .add_startup_system(setup_dataset)
        .run();
}

fn setup_dataset(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut path = home::home_dir().unwrap();
    path.push("Datasets/kitti/raw/");

    let dataset = kitti::RawDatasetExtractor::new(path);
    let frames = dataset.frames().collect::<Vec<kitti::RawDatasetFrame>>();

    let image = png2img(&frames[0].image);
    let image_aspect_ratio = image.aspect_2d();

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(image)),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 8.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * image_aspect_ratio,
    ))));

    // textured quad - normal
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.5),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn png2img(path: &PathBuf) -> Image {
    let bytes = fs::read(path.to_owned()).unwrap();
    Image::from_buffer(
        bytes.as_ref(),
        ImageType::MimeType("image/png")).unwrap()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cubes
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..Default::default()
    });
}
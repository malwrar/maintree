use std::fs;
use std::path::{Path, PathBuf};

use bevy::{
    render::texture::{Image, ImageType},
    prelude::*,
};

use malicious::kitti;

fn hello_world() {
    println!("Hello world.");
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world)
        .add_startup_system(setup_dataset)
        //.add_startup_system(setup)
        .run();
}

//fn png2img(path: &PathBuf) -> Image {
//    let image = Image::new(
//        Extend3d {
//            width: 1
//        }
//    )
//
//    image
//}

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

    let bytes = fs::read(frames[0].image.to_owned()).unwrap();
    let image = Image::from_buffer(
        bytes.as_ref(),
        ImageType::MimeType("image/png")).unwrap();

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 8.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * image.aspect_2d(),
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(image)),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // set up the camera
    // let mut camera = OrthographicCameraBundle::new_3d();
    // camera.orthographic_projection.scale = 3.0;
    // camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    // // camera
    // commands.spawn_bundle(camera);


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
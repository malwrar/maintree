extern crate bevy;
extern crate malicious;

use std::path::PathBuf;

use bevy::{
    render::{
        options::WgpuOptions,
        render_resource::{PrimitiveTopology, WgpuFeatures},
    },
    prelude::*,
};

use malicious::{
    bevy::pointcloud::{PointCloud, PointCloudConfig, PointCloudPlugin},
    kitti,
};

fn genmesh() -> Mesh {
    let points = kitti::parse_raw_velodyne(PathBuf::from("/home/sushi/Datasets/kitti/raw/velodyne_points/data/0000000000.bin"));

    let positions = points
        .iter()
        .map(|x| [x[0], x[1], x[2]])
        .collect::<Vec<[f32; 3]>>();

    let normals = (0..points.len())
        .map(|_| [0.0, 0.0, 1.0])
        .collect::<Vec<[f32; 3]>>();

    let uvs = (0..points.len())
        .map(|_| [0.0, 1.0])
        .collect::<Vec<[f32; 2]>>();

    let mut mesh = Mesh::new(PrimitiveTopology::PointList);
    mesh.set_indices(None);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}

fn setup_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pointcloud_config: ResMut<PointCloudConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    pointcloud_config.global = false;

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // textured quad - normal
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(genmesh()),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(PointCloud);

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 30.5, -50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    //env_logger::Builder::from_default_env()
    //    .filter_level(log::LevelFilter::Debug)
    //    .init();

    //log::info!("Starting up...");
    //log::warn!("Starting up...");
    //log::debug!("Starting up...");
    //log::error!("Starting up...");

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_POINT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PointCloudPlugin)
        .add_startup_system(setup_points)
        .run();
}
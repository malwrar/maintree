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

use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use malicious::{
    bevy::pointcloud::{PointCloud, PointCloudConfig, PointCloudPlugin},
    kitti,
};

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    log::info!("Setting up camera...");

    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-2.0, 5.0, 5.0),
        Vec3::new(0., 0., 0.),
    ));
}

fn genmesh() -> Mesh {
    let points = kitti::parse_raw_velodyne(PathBuf::from("/home/sushi/Datasets/kitti/raw/velodyne_points/data/0000000000.bin"));
    //let points = kitti::parse_raw_velodyne_dir(PathBuf::from("/home/sushi/Datasets/kitti/raw/velodyne_points"))
    //    .fold(Vec::new(), |mut acc, item| { acc.extend(item.data); acc });

    let positions = points
        .nearest(&[0.0, 0.0, 0.0])
        .map(|(_distance, point, _err)| point)
        .collect::<Vec<[f32; 3]>>();

    let normals = (0..points.size())
        .map(|_| [0.0, 0.0, 1.0])
        .collect::<Vec<[f32; 3]>>();

    let uvs = (0..points.size())
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
    log::info!("Setting up points...");

    pointcloud_config.global = false;

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
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pointcloud_config: ResMut<PointCloudConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    log::info!("Starting up...");
    log::warn!("Starting up...");
    log::debug!("Starting up...");
    log::error!("Starting up...");
}


fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_POINT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PointCloudPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_points)
        .run();
}
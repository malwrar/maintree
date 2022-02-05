use bevy::{
    render::{
        mesh::Indices,
        options::WgpuOptions,
        render_resource::{PrimitiveTopology, WgpuFeatures},
    },
    prelude::*,
};

use malicious::{
    bevy::pointcloud::{PointCloud, PointCloudConfig, PointCloudPlugin}
};

fn main() {
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
        transform: Transform::from_xyz(0.0, 3.5, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn genmesh() -> Mesh {
    let positions = vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    let normals = vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]];
    let uvs = vec![[0.0, 1.0], [0.0, 1.0], [0.0, 1.0]];

    let mut mesh = Mesh::new(PrimitiveTopology::PointList);
    mesh.set_indices(None);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}
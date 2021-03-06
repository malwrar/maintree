use std::fs;
use std::io;
use std::path::PathBuf;

use bevy::{
    render::{
        mesh::Indices,
        texture::{Image, ImageType},
        render_resource::PrimitiveTopology,
    },
    prelude::*,
};

use malicious::{
    kitti,
    bevy::{PointCloudPlugin, CameraPlugin},
    //prelude::*,
};

fn hello_world() {
    println!("Hello world.");
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //.add_plugin(CameraPlugin {})
        //.add_startup_system(hello_world)
        .add_startup_system(setup_points)
        //.add_startup_system(setup_dataset)
        .run();
}

fn setup_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    let mut path = home::home_dir().unwrap();
    path.push("Datasets/kitti/raw/velodyne_points/data/0000000000.bin");
    let mesh = points2mesh(&path);

    // textured quad - normal
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

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

fn points2mesh(path: &PathBuf) -> Mesh {
    //let f = fs::File::open(path).unwrap();
    //let mut reader = io::BufReader::new(f);
    //let mut points: Vec<[f32; 3]> = kitti::parse_raw_velodyne(&mut reader)
    //    .iter()
    //    .map(|vec| [vec.x, vec.y, vec.z])
    //    .collect();

    //points.resize(12, [0.0, 0.0, 0.0]); // HACK: truncate to match vbuffer size limit
    //println!("{:?}", points);


    let positions = vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    let normals = vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]];
    let uvs = vec![[0.0, 1.0], [0.0, 1.0], [0.0, 1.0]];
    let indices = Indices::U32(vec![0, 1, 2]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}
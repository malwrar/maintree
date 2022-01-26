use bevy::prelude::*;

use malicious::kitti;

fn hello_world() {
    println!("Hello world.");
}

fn extract_dataset() {
    let mut path = home::home_dir().unwrap();
    path.push("Datasets/kitti/raw/");

    let dataset = kitti::RawDatasetExtractor::new(path);
    //let frames = dataset.frames().collect::<Vec<kitti::RawDatasetFrame>>();
    for frame in dataset.frames() {
        println!("{:?}", frame);
    }
}

fn main() {
    App::new()
        //.insert_resource(Msaa { samples: 4 })
        //.add_plugins(DefaultPlugins)
        .add_startup_system(hello_world)
        .add_startup_system(extract_dataset)
        //.add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    // camera
    commands.spawn_bundle(camera);

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
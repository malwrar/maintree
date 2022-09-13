//! 3d renderer for 

use std::time::Duration;

use crossbeam_channel::{bounded, Receiver};

use bevy::{
    prelude::*,
    ecs::system::Resource,
    render::camera::Projection,
};

#[derive(Clone, Copy, Debug)]
struct Pose {
    foo: i32,
}

impl Default for Pose {
    fn default() -> Self {
        Self { foo: 0 }
    }
}

#[derive(Deref)]
struct PoseReceiver(Receiver<Pose>);
struct PoseEvent(Pose);

fn setup_tracker(mut commands: Commands) {
    let (tx, rx) = bounded::<Pose>(1);

    std::thread::spawn(move || {

        // TODO: poll tracker here?

        for i in 1..10 {
            std::thread::sleep(Duration::from_secs(1));
            tx.send(Pose { foo: i })
                .expect("Failed to send pose update.");
        }
    });

    commands.insert_resource(PoseReceiver(rx));
}

fn read_from_tracker(
    receiver: ResMut<PoseReceiver>,
    mut events: EventWriter<PoseEvent>
) {
    for pose in receiver.try_iter() {
        events.send(PoseEvent(pose));
    }
}

fn setup(
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

    // camera
    let projection = Projection::default();

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection,
        ..default()
    });
}

fn update(
    mut pose_updates: EventReader<PoseEvent>,
) {
    for e in pose_updates.iter() {
        let pose = e.0;

        println!("{:?}", pose);
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_event::<PoseEvent>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_tracker)
        .add_system(read_from_tracker)
        .add_system(update)
        .run();
}
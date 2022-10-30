//! 3d renderer for 

use bevy::{
    prelude::*,
    window::PresentMode,
};

use vision::{
    bevy_plugin::{CvPlugin, CvEvent},
};

use bevy_sandbox::{
    debug::DebugPlugin,
    debug_camera::{
        CameraController,
        CameraControllerPlugin
    },
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "3d Visualization".to_string(),
            width: 600.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(CvPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(setup_scene)
        .add_system(update)
        .run();
}

#[derive(Component, Default)]
struct Observer;

#[derive(Component, Default)]
struct Object;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(25.0, 10.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(CameraController::default());


    // cube
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    })
    .insert(Observer);
}

fn update(
    mut cv_events: EventReader<CvEvent>,
    mut query: Query<(&mut CameraController, &mut Transform)>,
) {
    for e in cv_events.iter() {

        println!("Updating pose {:?}", e);

        let (_, mut transform) = query.iter_mut().last().unwrap();

        match e {
            CvEvent::ViewerPoseUpdate(pose) => {
                let t = pose.translation;
                let t = Vec3::new(t.x, t.y, -t.z);

                *transform = transform
                    .with_translation(t);
            },
            //e => println!("Unhandled CvEvent: {:?}", e),
        }
    }
}
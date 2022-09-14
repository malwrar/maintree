//! 3d renderer for 

use rand::Rng;

use bevy::{
    prelude::*,
    render::camera::Projection,
    window::PresentMode,
};

use crossbeam_channel::{bounded, Receiver};

use opencv::{
	highgui,
	prelude::*,
	videoio,
};

use vision::{
    calibration::CameraCalibration,
    tracking::Tracker,
};

#[derive(Clone, Copy, Debug)]
struct Pose {
    translation: Vec3,
    rotation: Vec3,
}

impl Default for Pose {
    fn default() -> Self {
        Self { 
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Deref)]
struct PoseReceiver(Receiver<Pose>);
struct PoseEvent(Pose);

#[derive(Component)]
struct ARCamera {}

fn setup_tracker(mut commands: Commands) {
    let (tx, rx) = bounded::<Pose>(1);

    std::thread::spawn(move || {
    	let window = "video capture";
    	highgui::named_window(window, 1)
            .expect("Failed to create debug window!");
        
        let mut file = videoio::VideoCapture::from_file("./assets/tracking_test_1.mp4", videoio::CAP_ANY)
            .expect("Failed to open video file.");
        
        let calib = CameraCalibration::from_file(String::from("./assets/tracking_test_1.calib.yaml"))
            .expect("Failed to open calibration file.");
        
    	if !videoio::VideoCapture::is_opened(&file)
                .expect("Failed to open file.") {
    		panic!("Unable to open file!");
    	}
    
        let tracker = Tracker::new(calib);

    	loop {
    		if highgui::wait_key(10).expect("") > 0 { break; }

    		let mut frame = Mat::default();
    		if !file.read(&mut frame).expect("Failed to read next frame.") {
                break;
            }
        
            let mut tvec = Mat::default();
            let mut rvec = Mat::default();

            if tracker.track(&mut frame, &mut tvec, &mut rvec) {

                let translation = Vec3::new(
                        *tvec.at_2d::<f64>(0, 0).unwrap() as f32,
                        *tvec.at_2d::<f64>(1, 0).unwrap() as f32,
                        *tvec.at_2d::<f64>(2, 0).unwrap() as f32);

                let rotation = Vec3::new(
                        (*tvec.at_2d::<f64>(0, 0).unwrap() as f32).to_radians(),
                        (*tvec.at_2d::<f64>(1, 0).unwrap() as f32).to_radians(),
                        (*tvec.at_2d::<f64>(2, 0).unwrap() as f32).to_radians());

                tx.send(Pose {
                        translation,
                        rotation,
                    })
                    .expect("Failed to send pose update.");
            }
        

            highgui::imshow(window, &frame)
                .expect("Failed to show frame on debug window!");
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
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
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
    }).insert(ARCamera {

    });
}

fn update(
    mut pose_updates: EventReader<PoseEvent>,
    mut query: Query<(&mut ARCamera, &mut Transform)>,
) {
    for e in pose_updates.iter() {
        let pose = e.0;

        println!("Updating pose {:?}", pose);

        for (_camera, mut transform) in query.iter_mut() {
            *transform = transform
                .with_translation(pose.translation)
                .with_rotation(Quat::from_scaled_axis(pose.rotation));
                //.looking_at(Vec3::ZERO, Vec3::Y);
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Tracked cube position".to_string(),
            width: 600.,
            height: 600.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_event::<PoseEvent>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_tracker)
        .add_system(read_from_tracker)
        .add_system(update)
        .run();
}
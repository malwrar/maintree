use rand::Rng;

use bevy::{
    prelude::*,
    app::Plugin,
};

use crossbeam_channel::{bounded, Receiver};

#[derive(Default)]
pub struct CvPlugin;

impl Plugin for CvPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CvEvent>()
            .add_startup_system(setup_processing_thread)
            .add_system(forward_events_from_processing_thread);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pose {
    pub translation: Vec3,
    pub rotation: Vec3,
}

impl Default for Pose {
    fn default() -> Self {
        Self { 
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

/// Types of state changes we can send from the cv processing thread to the
/// renderer.
#[derive(Clone, Copy, Debug)]
pub enum CvEvent {
    ViewerPoseUpdate(Pose),
}

#[derive(Deref)]
struct CvEventReceiver(Receiver<CvEvent>);

/// Start processing thread that forwards messages to bevy via a crossbeam
/// channel.
fn setup_processing_thread(mut commands: Commands) {
    let (tx, rx) = bounded::<CvEvent>(1);

    std::thread::spawn(move || {
        loop {
            let pose = Pose {
                translation: Vec3::new(0.0,
                    rand::thread_rng().gen_range(1.0..10.0),
                    rand::thread_rng().gen_range(1.0..10.0)),
                ..Default::default()
            };

            tx.send(CvEvent::ViewerPoseUpdate(pose)).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    commands.insert_resource(CvEventReceiver(rx));
}

// Read `CvEvent`s from processing thread via crossbeam channel and forward
// them to a bevy EventWriter.
fn forward_events_from_processing_thread(
    receiver: ResMut<CvEventReceiver>,
    mut events: EventWriter<CvEvent>
) {
    for pose in receiver.try_iter() {
        events.send(pose);
    }
}
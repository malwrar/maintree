use bevy::{
    prelude::*,
    app::Plugin,
};

use crossbeam_channel::{bounded, Receiver};

use opencv::{
    calib3d,
    core::{
        Point2i,
        Point3f,
        Scalar,
    },
    highgui,
    imgproc,
    prelude::*,
    types::{
        VectorOfPoint3f,
        VectorOfPoint2f,
    },
    videoio,
    Result,
};

use crate::{
    calibration::CameraCalibration,
    tracking::Tracker,
};

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
        // Setup debug window that shows input footage.
        let window = "Observer playback";
        highgui::named_window(window, highgui::WINDOW_NORMAL)
            .expect("Failed to create debug window!");

        // When footage ends, repeat playback.
        loop {
            // Open input video feed & metadata.
            let mut file = videoio::VideoCapture::from_file("./assets/office_calib_iphone/translate_left_right.mov", videoio::CAP_ANY)
                .expect("Failed to open video file.");

            let calib = CameraCalibration::from_file(String::from("./assets/office_calib_iphone/translate_left_right.yaml"))
                .expect("Failed to open calibration file.");

            if !videoio::VideoCapture::is_opened(&file)
                   .expect("Failed to open file.") {
                panic!("Unable to open file!");
            }

            // Setup cv resources.
            let tracker = Tracker::new(calib);

            loop {
                if highgui::wait_key(10).expect("") > 0 { break; }

                // Read next frame.
                let mut frame = Mat::default();
                if !file.read(&mut frame).expect("Failed to read next frame.") {
                    break;
                }

                // Try to (re)locate points, lines, & regions, use these to
                // estimate the observer's pose.
                let mut rvec = Mat::default();
                let mut tvec = Mat::default();

                if tracker.track(&frame, &mut rvec, &mut tvec) {
                    let translation = Vec3::new(
                            -*tvec.at_2d::<f64>(0, 0).unwrap() as f32,
                            *tvec.at_2d::<f64>(2, 0).unwrap() as f32,
                            *tvec.at_2d::<f64>(1, 0).unwrap() as f32);

                    let rotation = Vec3::new(
                            *rvec.at_2d::<f64>(0, 0).unwrap() as f32,
                            *rvec.at_2d::<f64>(2, 0).unwrap() as f32,
                            *rvec.at_2d::<f64>(1, 0).unwrap() as f32);

                    let pose = Pose {
                        translation,
                        rotation,
                        ..Default::default()
                    };

                    // Send pose update to the renderer.
                    tx.send(CvEvent::ViewerPoseUpdate(pose)).unwrap();
                }

                // HACK: shrink window while retaining aspect ratio.
                let aspect_ratio = frame.rows() as f32 / frame.cols() as f32;
                let width = 600.0;
                highgui::resize_window(window, width as i32,
                        (width * aspect_ratio) as i32)
                    .unwrap();

                highgui::imshow(window, &frame)
                    .unwrap();
            }
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
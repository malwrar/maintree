//!

use opencv::{
	highgui,
	prelude::*,
	videoio,
};

use vision::{
    calibration::CameraCalibration,
    tracking::Tracker,
};


fn main() {
	let window = "video capture";
	highgui::named_window(window, 1)
        .expect("Failed to create debug window!");

    let mut file = videoio::VideoCapture::from_file("./assets/tracking_test_1.mp4", videoio::CAP_ANY)
        .expect("Failed to open video file.");

    let mut calib = CameraCalibration::from_file(String::from("./assets/tracking_test_1.calib.yaml"))
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

        tracker.annotate_frame(&mut frame);

        highgui::imshow(window, &frame)
            .expect("Failed to show frame on debug window!");
	}
}
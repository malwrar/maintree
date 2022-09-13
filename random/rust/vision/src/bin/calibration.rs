//! Estimate camera's focal length (f_x, f_y), optical center (c_x, c_y), and
//! nonlinear distortion parameters by tracking calibration patterns.

use opencv::{
	prelude::*,
	core::{
		Point2f,
		Point2i,
		Point3f,
		Scalar,
		Size,
		TermCriteria,
		TermCriteria_Type,
	},
	calib3d,
	highgui,
	imgproc,
    videoio,
};

use clap::Parser;

use vision::{
    calibration::CameraCalibration,
    pattern::{Chessboard, Pattern},
};

fn preprocess_image(image: &Mat) -> Mat {
	let mut gray = Mat::default();
	imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    gray
}

// 
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    // Input video file containing a camera recording capturing a calibration pattern
    #[clap(short='f', long="in", value_parser)]
    video_file: String,

    // Output file where camera calibration & ground truth data should be recorded
    #[clap(short='o', long="out", value_parser)]
    calib_file: String,

    /*
    // Number of rows in the calibration chessboard
    #[clap(short='r', long, value_parser)]
    chessboard_rows: u32,

    // Number of rows in the calibration chessboard
    #[clap(short='c', long, value_parser)]
    chessboard_cols: u32,

    // Replay input
    #[clap(long, value_parser)]
    verify: bool,
    */
}

fn main() {
    //Args::parse();

	let window = "video capture";
	highgui::named_window(window, 1)
        .expect("Failed to create debug window!");

    let mut file = videoio::VideoCapture::from_file("./recording.mp4", videoio::CAP_ANY)
        .expect("Failed to open source file!");

	if !videoio::VideoCapture::is_opened(&file)
            .expect("Failed to check if file open!") {
		panic!("Unable to open source file!");
	}

    // Find and record all chessboard info in the provided input.
	let mut observation_count = 0u64;
    let mut object_points: Vec<Vec<Point3f>> = Vec::new();
    let mut image_points: Vec<Vec<Point2f>> = Vec::new();
    let mut capture_width = 0;
    let mut capture_height = 0;

	loop {
		if highgui::wait_key(10).expect("Failed to wait for key!") > 0 { break; }
		//if observation_count > 20 { break; }  // TODO: eliminate this when structure figured out.

		let mut frame = Mat::default();
		if !file.read(&mut frame).expect("Failed to read next frame!") {
            break;
        }

        capture_width = capture_width.max(frame.cols());
        capture_height = capture_height.max(frame.rows());

        let preprocessed = preprocess_image(&frame);

        for (points_2d, board) in Chessboard::find(&preprocessed, 9, 6) {
			observation_count += 1;

            for pt in &points_2d {
                imgproc::circle(&mut frame,
                        Point2i::new(pt.x as i32, pt.y as i32), 1,
                        Scalar::new(255.0, 255.0, 0.0, 255.0), 2, 0, 0)
                    .expect("Failed to draw circle!");
            }

            //image_points.push(VectorOfPoint2f::from_iter(points_2d.iter().map(|e| e.)));
            object_points.push(board.points());
            image_points.push(points_2d);
        }


        // TODO: annotate frame with observation_count
        highgui::imshow(window, &frame)
            .expect("Failed to show frame on debug window!");
	}

    println!("Attempting calibration...");
    let mut rvecs = Mat::default();
    let mut tvecs = Mat::default();
    let (calib, _map) = CameraCalibration::from_observations(capture_width,
            capture_height, object_points, image_points, &mut rvecs,
            &mut tvecs)
        .expect("Failed to calibrate camera!");

    calib.write_to_file(String::from("./recording.calib.yaml"))
        .expect("Failed to create calib file.");
}
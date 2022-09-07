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
};

trait Pattern {
    fn points(&self) -> Vec<Point3f>;
}

struct Chessboard {
    width: i32,
    height: i32,
}

impl Chessboard {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn find(image: &Mat,
        width: i32,
        height: i32,
    ) -> Vec<(Vec<Point2f>, Self)> {
        let mut boards = Vec::new();

        let mut points_2d = Mat::default();
        let found = calib3d::find_chessboard_corners(&image,
                Size::new(width, height),
				&mut points_2d, calib3d::CALIB_CB_ADAPTIVE_THRESH
				    + calib3d::CALIB_CB_NORMALIZE_IMAGE
					+ calib3d::CALIB_CB_FAST_CHECK)
            .unwrap();

        if found {
            let board = Chessboard::new(width, height);

            // Refine pixel positions
			let subpix_criteria = TermCriteria::new(
					TermCriteria_Type::EPS as i32 | TermCriteria_Type::COUNT as i32,
					30, 0.001).unwrap();

			imgproc::corner_sub_pix(&image, &mut points_2d, Size::new(11, 11),
                    Size::new(-1, -1), subpix_criteria)
				.unwrap();

            // Convert to VectorOfPoint2f

            let points_2d_vec: Vec<Point2f> = points_2d
                .to_vec_2d()
                .expect("Failed to convert points Mat to Vec.")
                .iter()
                .flat_map(|v| v.iter())
                .cloned()
                .collect();

            boards.push((points_2d_vec, board));
        }

        boards
    }
}

impl Pattern for Chessboard {
    fn points(&self) -> Vec<Point3f> {
        let mut points = Vec::new();

		for x in 0..self.height {
			for y in 0..self.width {
				points.push(Point3f::new(x as f32, y as f32, 0.0));
			}
		}

        points
    }
}

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

    let mut file = videoio::VideoCapture::from_file("./assets/tracking_test_1.mp4", videoio::CAP_ANY)
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
		if observation_count > 20 { break; }  // TODO: eliminate this when structure figured out.

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

    calib.write_to_file(String::from("./assets/tracking_test_1.calib.yaml"))
        .expect("Failed to create calib file.");
}
//! Tracks the chessboard and QR code patterns in tracking_test_1.mp4.
//! Based on https://docs.opencv.org/4.6.0/de/d45/samples_2cpp_2tutorial_code_2features2D_2Homography_2decompose_homography_8cpp-example.html

use opencv::{
	core::{Point2f, Point3f, Size, CV_64F, TermCriteria, TermCriteria_Type},
	calib3d,
	highgui,
	imgproc,
	prelude::*,
	Result,
	videoio,
};

use petgraph::{
	graph::UnGraph,
};

const BOARD_WIDTH: i32 = 9;
const BOARD_HEIGHT: i32 = 6;
const SQUARE_SIZE: f32 = 4.0;

//fn draw_axis(image: &Mat, R: &Mat, t: &Mat, K: &Mat, dist_coef, &Mat)  -> Result<()> {
//	let mut rot_v = Mat::default();
//	let mut jacobian_ = Mat::default();
//
//	calib3d::rodrigues(&R, &mut rot_v, &mut jacobian_)?;
//
//	let points: Vec<Vec<f32>> = vec![
//		vec![100.0, 0.0, 0.0],
//		vec![0.0, 100.0, 0.0],
//		vec![0.0, 0.0, 100.0],
//		vec![0.0, 0.0, 0.0],
//	];
//	let points = Mat::from_slice_2d(&points)?.reshape(-1, 3)?;
//
//	println!("{:?}", points);
//
//	//imgproc::line(&mut image)
//
//	Ok(())
//}

//fn calc_chessboard_corners() ->

struct Chessboard {
	pub corners_2d: Mat,
	pub corners_3d: Vec<Point3f>,
}

impl Chessboard {
	fn find(frame: &Mat) -> Option<Self> {
		let mut corners = Mat::default();

		if (!calib3d::find_chessboard_corners(
				&frame, Size::new(BOARD_WIDTH, BOARD_HEIGHT),
				&mut corners, 
				calib3d::CALIB_CB_ADAPTIVE_THRESH + calib3d::CALIB_CB_NORMALIZE_IMAGE
					+ calib3d::CALIB_CB_FAST_CHECK).unwrap()) {
			return None;
		}

		let subpix_criteria = TermCriteria::new(
			TermCriteria_Type::EPS as i32 | TermCriteria_Type::COUNT as i32,
			30, 0.001).unwrap();

		imgproc::corner_sub_pix(&frame, &mut corners, Size::new(11, 11), Size::new(-1, -1), subpix_criteria).unwrap();

		Some(Self {
			corners_2d: corners,
			corners_3d: Vec::new(),  // Need more observations to be located.
		})
	}
}

struct State {
	pub observations: Vec<Chessboard>,
	//pub views: UnGraph::<u64, ()>,
	//pub K: Mat,
	//pub dist_coef: Mat,
}

impl State {
	pub fn new() -> Self {
		//let K = Mat::eye_size(Size::new(3, 3), CV_64F).unwrap();
        //K.at<double>(0, 0) = fx;
        //K.at<double>(1, 1) = fy;
        //K.at<double>(0, 2) = cx;
        //K.at<double>(1, 2) = cy;

		//let dist_coef = Mat::eye_size(Size::new(4, 1), CV_64F).unwrap();
        //dist_coef = cv::Mat(4, 1, CV_64F);
        //dist_coef.at<double>(0) = k1;
        //dist_coef.at<double>(1) = k2;
        //dist_coef.at<double>(2) = p1;
        //dist_coef.at<double>(3) = p2;

		Self {
			observations: Vec::new(),
			//views: UnGraph::new(),
			//K,
			//dist_coef,
		}
	}

	pub fn process_next(&mut self, frame: &Mat) -> bool {
		let mut gray = Mat::default();
		imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

		if let Some(chessboard) = Chessboard::find(&gray) {
			self.add_observation(chessboard);
			return true;
		}

		false
	}

	pub fn add_observation(&mut self, chessboard: Chessboard) {
		self.observations.push(chessboard);
	}
}

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, 1)?;

    let mut file = videoio::VideoCapture::from_file("./assets/tracking_test_1.mp4", videoio::CAP_ANY)?;

	if !videoio::VideoCapture::is_opened(&file)? {
		panic!("Unable to open camera!");
	}

	let mut state = State::new();

	loop {
		if highgui::wait_key(10)? > 0 { break; }

		let mut frame = Mat::default();
		file.read(&mut frame)?;

		if state.process_next(&frame) {
			let observation = &state.observations[state.observations.len() - 1];
			calib3d::draw_chessboard_corners(
				&mut frame,
				Size::new(BOARD_WIDTH, BOARD_HEIGHT),
				&observation.corners_2d,
				true);

			println!("Found chessboard ({} so far)!", state.observations.len());
		}

		//draw_axis(&mut frame, &R, &t, &K, &dist_coef);

        highgui::imshow(window, &frame)?;
	}

	Ok(())
}
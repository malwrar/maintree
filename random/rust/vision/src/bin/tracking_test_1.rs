//! Tracks the chessboard and QR code patterns in tracking_test_1.mp4.
//! Based on https://docs.opencv.org/4.6.0/de/d45/samples_2cpp_2tutorial_code_2features2D_2Homography_2decompose_homography_8cpp-example.html

use std::collections::{
	HashMap,
	LinkedList,
};

use opencv::{
	core::{
		Point2f,
		Point3f,
		Size,
		CV_64F,
		TermCriteria,
		TermCriteria_Type
	},
	calib3d,
	highgui,
	imgproc,
	prelude::*,
	Result,
	videoio,
};

use petgraph::{
	Graph,
	graph::NodeIndex,
	dot::Dot, visit::IntoNeighbors,
};

use uuid::Uuid;

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

#[derive(Debug)]
struct Chessboard {
	pub points_3d: Mat,
	pub points_2d: Mat,
	// TODO: adapt to the following
	//   1. "Chessboard" is a collection of corners, which can be thought of as a planar collection of 3d points with a known true orientation.
	//   2. Chessboard's known 3d points are found in 2d image c1, we can now solvePnP for t[] and R[]
	//   3. Draw axis with PnP-derived t[] and R[]
	//   4. Repeat for next image c2
	//   5. Calculate camera displacement between c1 and c2 using t[] and R[], use this to estimate c2 pose.
	//   6. Send c1, c2, ..., pose to bevy visualization, use to move camera initially facing cube
	//   7. dv/dt prediction for interpolation/smoothing?
	//   8. What assumptions are we making? Can we estimate object warp and move chessboard 3d points accordingly?
}

impl Chessboard {
	fn new() -> Self {
		// Create normalized 3d representation of board corner locations.
		let mut points_3d = Vec::new();
		for x in 0..BOARD_WIDTH {
			for y in 0..BOARD_HEIGHT {
				points_3d.push(vec![x as f32, y as f32, 0.0]);
			}
		}

		// Convert that to something opencv-compatible.
		let points_3d = Mat::from_slice_2d(&points_3d)
			.unwrap()
			.reshape(1, 3)
			.unwrap();

		Self {
			points_3d,
			points_2d: Mat::default(),
		}
	}

	fn find(frame: &Mat) -> Vec<Self> {
		// TODO: ugly...
		let mut boards = Vec::new();

		let mut board = Self::new();
		if (calib3d::find_chessboard_corners(&frame,
				Size::new(BOARD_WIDTH, BOARD_HEIGHT),
				&mut board.points_2d, calib3d::CALIB_CB_ADAPTIVE_THRESH
				    + calib3d::CALIB_CB_NORMALIZE_IMAGE
					+ calib3d::CALIB_CB_FAST_CHECK).unwrap()) {
			let subpix_criteria = TermCriteria::new(
					TermCriteria_Type::EPS as i32 | TermCriteria_Type::COUNT as i32,
					30, 0.001).unwrap();

			imgproc::corner_sub_pix(&frame, &mut board.points_2d, Size::new(11, 11),
					Size::new(-1, -1), subpix_criteria)
				.unwrap();		

			// TODO: adapt 3d point mesh to observed warped surface?

			boards.push(board);
		}

		boards
	}

	//fn find(frame: &Mat) -> Option<Self> {
	//	let mut corners = Mat::default();
	//	if (!calib3d::find_chessboard_corners(
	//			&frame, Size::new(BOARD_WIDTH, BOARD_HEIGHT),
	//			&mut corners, 
	//			calib3d::CALIB_CB_ADAPTIVE_THRESH
	//			    + calib3d::CALIB_CB_NORMALIZE_IMAGE
	//				+ calib3d::CALIB_CB_FAST_CHECK).unwrap()) {
	//		return None;
	//	}

	//	let subpix_criteria = TermCriteria::new(
	//		TermCriteria_Type::EPS as i32 | TermCriteria_Type::COUNT as i32,
	//		30, 0.001).unwrap();
	//	imgproc::corner_sub_pix(&frame, &mut corners, Size::new(11, 11), Size::new(-1, -1), subpix_criteria).unwrap();

	//	Some(Self {
	//		corners_2d: corners,
	//		corners_3d: Vec::new(),  // Need more observations to be located.
	//	})
	//}
}

/**
* Contains info about the state of an observer viewing a scene at some point in
* time.
*/
#[derive(Debug)]
pub struct Sample {

}

#[derive(Debug)]
enum Observation {
	KeyFrame,
	Landmark(Uuid, Chessboard),
}

/**
 * Reconstructs a world as viewed by some observer represented as a continuous
 * feed of monocular camera images.
 * 
 * In other words if you feed this class video recordings or live camera input
 * it'll estimate and concurrently refine the camera's pose as it travel(s/ed)
 * through the scene and various observed 3d landmarks.
 */
struct World {
	pub observations: Graph<Observation, ()>, // Stuff we've seen at the places we've been.
	last_sample: Option<NodeIndex>,
	pub landmarks: HashMap<Uuid, NodeIndex>,
	//pub nearby: Graph<Uuid, Uuid>,              // Landmarks observed near other landmarks.
	//pub K: Mat,
	//pub dist_coef: Mat,
}

impl World {
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
			observations: Graph::new(),
			last_sample: None,
			landmarks: HashMap::new(),
			//nearby: Graph::new(),
			//K,
			//dist_coef,
		}
	}

	pub fn process_next(&mut self, frame: &Mat) -> bool {
		let mut gray = Mat::default();
		imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

		let landmarks = self.find_landmarks(&gray);
		if landmarks.len() > 0 {
			let keyframe_idx = self.add_keyframe();

			for (uuid, landmark) in landmarks {
				let points_2d = landmark.points_2d.clone(); // Make a copy of this before landmark ownership changes.

				let landmark_idx = self.landmarks
					.entry(uuid)
					.or_insert_with(|| self.observations.add_node(Observation::Landmark(uuid, landmark)))
					.clone();

				let node = self.observations.node_weight_mut(landmark_idx).unwrap();
				match node {
					Observation::Landmark(_, data) => {
						data.points_2d = points_2d;
					},
					_ => (),
				}

				self.observations.add_edge(keyframe_idx, landmark_idx, ());
			}

			return true;
		}

		false
	}
	
	pub fn debug_annotate_last(&self, frame: &mut Mat) {
		if let Some(sample) = self.last_sample {
			for node_idx in self.observations.neighbors(sample) {
				let observation = self.observations
					.node_weight(node_idx)
					.unwrap();

				match observation {
					Observation::Landmark(_uuid, landmark) => {
						calib3d::draw_chessboard_corners(frame,
							Size::new(BOARD_WIDTH, BOARD_HEIGHT),
							&landmark.points_2d, true)
						.unwrap();
					},
					_ => (),
				}
			}
		}
	}

	fn find_landmarks(&self, frame: &Mat) -> Vec<(Uuid, Chessboard)> {
		let mut found = Vec::new();

		for landmark in Chessboard::find(frame) {
			found.push(
				(
					// TODO; include an ID mechanism rather than assuming it's the same chessboard?
					Uuid::parse_str("1651e3b0-cd6e-4e22-bb7e-6fe98d213b49").unwrap(),
					landmark,
				)
			);
		}

		found
	}

	fn add_keyframe(&mut self) -> NodeIndex {
		let sample = self.observations.add_node(Observation::KeyFrame);

		if let Some(last_sample) = self.last_sample {
			self.observations.add_edge(last_sample, sample, ());
		}
		self.last_sample = Some(sample);

		sample
	}

}

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, 1)?;

    let mut file = videoio::VideoCapture::from_file("./assets/tracking_test_1.mp4", videoio::CAP_ANY)?;

	if !videoio::VideoCapture::is_opened(&file)? {
		panic!("Unable to open camera!");
	}

	let mut world = World::new();
	let mut observation_count = 0u64;

	loop {
		if highgui::wait_key(10)? > 0 { break; }
		//if observation_count > 5 { break; }  // TODO: eliminate this when structure figured out.

		let mut frame = Mat::default();
		file.read(&mut frame)?;

		if world.process_next(&frame) {
			observation_count += 1;
			

			//println!("Found chessboard ({} so far)!", state.observations.len());
			world.debug_annotate_last(&mut frame);
		}

		//draw_axis(&mut frame, &R, &t, &K, &dist_coef);

        highgui::imshow(window, &frame)?;
	}

	println!("{:?}", Dot::new(&world.observations));

	Ok(())
}
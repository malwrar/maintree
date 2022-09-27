//! Tool for realtime tracking of camera and environment state changes between
//! observations.

use crate::{
    mapping::{
        MetricMap,
        Pose,
    },
    calibration::CameraCalibration,
    pattern::{Chessboard, Pattern},
};

use opencv::{
	prelude::*,
	core::{
		Point2i,
		Point3f,
		Scalar,
		Size,
		TermCriteria,
		TermCriteria_Type,
	},
    types::{
        VectorOfPoint3f,
        VectorOfPoint2f,
    },
    Result,
	calib3d,
	highgui,
	imgproc,
    videoio,
};

fn preprocess_image(image: &Mat) -> Mat {
	let mut gray = Mat::default();
	imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    gray
}

pub struct Tracker {
    pub calib: CameraCalibration,
    pub map: MetricMap,
}

impl Tracker {
    pub fn new(calib: CameraCalibration) -> Self {
        Self {
            calib,
            map: MetricMap::new(),
        }
    }

    pub fn track(&self, frame: &Mat, rvec: &mut Mat, tvec: &mut Mat) -> bool {
        let preprocessed = preprocess_image(&frame);

		if let Some((points_2d, board)) = Chessboard::find(&preprocessed, 9, 6).last() {
            calib3d::solve_pnp(
                    &VectorOfPoint3f::from_iter(board.points()),
                    &VectorOfPoint2f::from_iter(points_2d.clone()),
                    &self.calib.k(),
                    &self.calib.dist_coeffs(),
                    rvec,
                    tvec,
                    false,
                    0)
                .expect("Failed to solve PnP problem.");

			return true;
		}

		false
	}
}
//!

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

pub trait Pattern {
    fn points(&self) -> Vec<Point3f>;
}

pub struct Chessboard {
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
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

fn draw_axis(image: &mut Mat, r: &Mat, t: &Mat, k: &Mat, dist_coeffs: &Mat)  -> Result<()> {
	// Define the virual 3d axis lines.
	let points = VectorOfPoint3f::from_slice(&[
		Point3f::new(1.0, 0.0, 0.0),  // [0]: x
		Point3f::new(0.0, 1.0, 0.0),  // [1]: y
		Point3f::new(0.0, 0.0, 1.0),  // [2]: z
		Point3f::new(0.0, 0.0, 0.0),  // [3]: origin
	]);

	let mut image_points = VectorOfPoint2f::new();
	let mut jacobian = Mat::default();
	calib3d::project_points(&points, &r, &t, &k, dist_coeffs,
			&mut image_points, &mut jacobian, 0.0)
		.unwrap();
	// TODO: inverse of PnP?? Make a test program with artificial data.

	// Palette
	let r = Scalar::new(255.0, 0.0, 0.0, 1.0);
	let g = Scalar::new(0.0, 255.0, 0.0, 1.0);
	let b = Scalar::new(0.0, 0.0, 255.0, 1.0);
	let c = Scalar::new(255.0, 255.0, 0.0, 1.0);

	// Points
	let image_points = image_points.as_slice();
	let p0 = image_points[0];
	let p1 = image_points[1];
	let p2 = image_points[2];
	let p3 = image_points[3];

	let x = Point2i::new(p0.x as i32, p0.y as i32);
	let y = Point2i::new(p1.x as i32, p1.y as i32);
	let z = Point2i::new(p2.x as i32, p2.y as i32);
	let origin = Point2i::new(p3.x as i32, p3.y as i32);

	imgproc::line(image, origin, x, r, 3, 0, 0).unwrap();
	imgproc::line(image, origin, y, g, 3, 0, 0).unwrap();
	imgproc::line(image, origin, z, b, 3, 0, 0).unwrap();
	
	// Draw the 2d points for good measure.
	//for p in image_points {
	//	imgproc::circle(image, Point2i::new(p.x as i32, p.y as i32), 1, c, 3, 0, 0).unwrap();
	//}

	Ok(())
}

fn draw_cube(image: &mut Mat, r: &Mat, t: &Mat, k: &Mat, dist_coeffs: &Mat)  -> Result<()> {
	// Define the virual 3d axis lines.
	let vertices = VectorOfPoint3f::from_slice(&[
		Point3f::new(0.0, 0.0, 0.0),
		Point3f::new(3.0, 0.0, 0.0),
		Point3f::new(0.0, 3.0, 0.0),
		Point3f::new(3.0, 3.0, 0.0),
		Point3f::new(0.0, 0.0, 3.0),
		Point3f::new(3.0, 0.0, 3.0),
		Point3f::new(0.0, 3.0, 3.0),
		Point3f::new(3.0, 3.0, 3.0),
	]);

	let indices = [
		// top
		(0, 1),
		(2, 3),
		(0, 2),
		(1, 3),

		// bottom
		(4, 5),
		(6, 7),
		(4, 6),
		(5, 7),

		// sides
		(0, 4),
		(1, 5),
		(2, 6),
		(3, 7),
	];

	let mut image_points = VectorOfPoint2f::new();
	let mut jacobian = Mat::default();
	calib3d::project_points(&vertices, &r, &t, &k, dist_coeffs,
			&mut image_points, &mut jacobian, 0.0)
		.unwrap();
	// TODO: inverse of PnP?? Make a test program with artificial data.

	// Palette
	let g = Scalar::new(0.0, 0.0, 255.0, 1.0);

	// Points
	let pts = image_points.as_slice();

	for (i, j) in indices {
	    imgproc::line(image,
				Point2i::new(pts[i].x as i32, pts[i].y as i32),
				Point2i::new(pts[j].x as i32, pts[j].y as i32),
				g, 2, 0, 0)
			.unwrap();
	}

	// Draw the 2d points for good measure.
	//for p in image_points {
	//	imgproc::circle(image, Point2i::new(p.x as i32, p.y as i32), 1, c, 3, 0, 0).unwrap();
	//}

	Ok(())
}

fn preprocess_image(image: &Mat) -> Mat {
	let mut gray = Mat::default();
	imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    gray
}

pub struct Tracker {
    calib: CameraCalibration,
    pub map: MetricMap,
}

impl Tracker {
    pub fn new(calib: CameraCalibration) -> Self {
        Self {
            calib,
            map: MetricMap::new(),
        }
    }

    pub fn annotate_frame(&self, frame: &mut Mat) {
        let preprocessed = preprocess_image(&frame);

        for (points_2d, board) in Chessboard::find(&preprocessed, 9, 6) {
            //for pt in &points_2d {
            //    //imgproc::circle(frame,
            //    //        Point2i::new(pt.x as i32, pt.y as i32), 1,
            //    //        Scalar::new(255.0, 255.0, 0.0, 255.0), 2, 0, 0)
            //    //    .expect("Failed to draw circle!");
            //}
            let k = self.calib.k();
            let dist_coeffs = self.calib.dist_coeffs();

            let mut rvec = Mat::default();
            let mut tvec = Mat::default();

            calib3d::solve_pnp(
                    &VectorOfPoint3f::from_iter(board.points()),
                    &VectorOfPoint2f::from_iter(points_2d),
                    &k,
                    &dist_coeffs,
                    &mut rvec,
                    &mut tvec,
                    false,
                    0)
                .expect("Failed to solve PnP problem.");

            draw_cube(frame, &rvec, &tvec, &k, &dist_coeffs)
                .expect("Failed to draw axis.");
        }
    }
}
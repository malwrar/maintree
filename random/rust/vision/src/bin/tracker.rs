//!

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

use plotters::prelude::*;

use vision::{
    calibration::CameraCalibration,
    tracking::Tracker,
};


fn main() {
	let window = "video capture";
	highgui::named_window(window, 1)
        .expect("Failed to create debug window!");

    let mut file = videoio::VideoCapture::from_file("./assets/office_calib_iphone/translate_left_right.mov", videoio::CAP_ANY)
        .expect("Failed to open video file.");

    let calib = CameraCalibration::from_file(String::from("./assets/office_calib_iphone/translate_left_right.yaml"))
        .expect("Failed to open calibration file.");

	if !videoio::VideoCapture::is_opened(&file)
            .expect("Failed to open file.") {
		panic!("Unable to open file!");
	}

    let tracker = Tracker::new(calib);
    let mut poses = Vec::new();

    let mut capture_width = 0;
    let mut capture_height = 0;

	loop {
		if highgui::wait_key(10).expect("") > 0 { break; }

		let mut frame = Mat::default();
		if !file.read(&mut frame).expect("Failed to read next frame.") {
            break;
        }

        capture_width = capture_width.max(frame.rows());
        capture_height = capture_height.max(frame.cols());

        let ratio = capture_height as f32 / capture_width as f32;
        highgui::resize_window(window, 80, (80.0*ratio) as i32)
            .expect("Failed to resize window.");

		let mut rvec = Mat::default();
		let mut tvec = Mat::default();

        if tracker.track(&frame, &mut rvec, &mut tvec) {
            draw_cube(&mut frame, &rvec, &tvec, &tracker.calib.k(), &tracker.calib.dist_coeffs())
                .expect("Failed to draw axis.");

            poses.push(tvec);
        }

        highgui::imshow(window, &frame)
            .expect("Failed to show frame on debug window!");
	}

    println!("Tracked {} poses, rendering graph.", poses.len());

    let root = BitMapBackend::new("translation_xy.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let min_x = poses.iter().fold(f64::INFINITY, |a, b| b.at::<f64>(0).unwrap().min(a));
    let max_x = poses.iter().fold(0.0, |a, b| b.at::<f64>(0).unwrap().max(a));
    let min_y = poses.iter().fold(f64::INFINITY, |a, b| b.at::<f64>(1).unwrap().min(a));
    let max_y = poses.iter().fold(0.0, |a, b| b.at::<f64>(1).unwrap().max(a));

    let mut chart = ChartBuilder::on(&root)
        .caption("translation xy", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
            .unwrap();

    chart
        .configure_mesh()
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            poses.iter().map(|t| (t.at::<f64>(0).unwrap().to_owned(), t.at::<f64>(1).unwrap().to_owned())),
            &RED,
        ))
        .unwrap()
        .label("translation");

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}

fn draw_cube(image: &mut Mat, r: &Mat, t: &Mat, k: &Mat, dist_coeffs: &Mat) -> Result<()> {
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

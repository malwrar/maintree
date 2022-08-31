//!

use opencv::{
	core::{Size, CV_64F, TermCriteria, TermCriteria_Type},
	calib3d,
	highgui,
	imgproc,
	prelude::*,
	Result,
	videoio,
};

fn draw_axis(image: &Mat, R: &Mat, t: &Mat, K: &Mat)  -> Result<()> {
	let mut rot_v = Mat::default();
	let mut jacobian_ = Mat::default();

	calib3d::rodrigues(&R, &mut rot_v, &mut jacobian_)?;

	let points: Vec<Vec<f32>> = vec![
		vec![100.0, 0.0, 0.0],
		vec![0.0, 100.0, 0.0],
		vec![0.0, 0.0, 100.0],
		vec![0.0, 0.0, 0.0],
	];
	let points = Mat::from_slice_2d(&points)?.reshape(-1, 3)?;

	println!("{:?}", points);

	//imgproc::line(&mut image)

	Ok(())
}

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, 1)?;

    let mut file = videoio::VideoCapture::from_file("./assets/tracking_test_1.mp4", videoio::CAP_ANY)?;

	if !videoio::VideoCapture::is_opened(&file)? {
		panic!("Unable to open camera!");
	}

	loop {
		if highgui::wait_key(10)? > 0 { break; }

		let mut frame = Mat::default();
		file.read(&mut frame)?;

		let mut gray = Mat::default();
		imgproc::cvt_color(
			&frame,
			&mut gray,
			imgproc::COLOR_BGR2GRAY,
			0)?;

		let mut corners = Mat::default();
		let found = calib3d::find_chessboard_corners(&gray, Size::new(9, 6), &mut corners, 
				calib3d::CALIB_CB_ADAPTIVE_THRESH + calib3d::CALIB_CB_NORMALIZE_IMAGE
				+ calib3d::CALIB_CB_FAST_CHECK)?;

		if found {
			let subpix_criteria = TermCriteria::new(TermCriteria_Type::EPS as i32 | TermCriteria_Type::COUNT as i32,
				30, 0.001)?;

			imgproc::corner_sub_pix(&gray, &mut corners, Size::new(11, 11), Size::new(-1, -1), subpix_criteria)?;
		}
		calib3d::draw_chessboard_corners(&mut frame, Size::new(9, 6), &corners, found);

		let K = Mat::eye_size(Size::new(3, 3), CV_64F);
        //K.at<double>(0, 0) = fx;
        //K.at<double>(1, 1) = fy;
        //K.at<double>(0, 2) = cx;
        //K.at<double>(1, 2) = cy;

		let dist_coef = Mat::eye_size(Size::new(4, 1), CV_64F);
        //dist_coef = cv::Mat(4, 1, CV_64F);
        //dist_coef.at<double>(0) = k1;
        //dist_coef.at<double>(1) = k2;
        //dist_coef.at<double>(2) = p1;
        //dist_coef.at<double>(3) = p2;

		//draw_axis(image: &Mat, R: &Mat, t: &Mat, &K)

        highgui::imshow(window, &frame)?;
	}

	Ok(())
}
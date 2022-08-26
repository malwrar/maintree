//!

use opencv::{
	highgui,
	prelude::*,
	Result,
	videoio,
};

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, 1)?;

	opencv::opencv_branch_32! {
		let mut cam = videoio::VideoCapture::new_default(0)?;
	}
	opencv::not_opencv_branch_32! {
		let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
	}

	if !videoio::VideoCapture::is_opened(&cam)? {
		panic!("Unable to open camera!");
	}

	loop {
		if highgui::wait_key(10)? > 0 { break; }

		let mut frame = Mat::default();
		cam.read(&mut frame)?;

        highgui::imshow(window, &frame)?;
	}

	Ok(())
}
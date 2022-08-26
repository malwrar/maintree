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

    let mut file = videoio::VideoCapture::from_file("./assets/tracking_test_1.mp4", videoio::CAP_ANY)?;

	if !videoio::VideoCapture::is_opened(&file)? {
		panic!("Unable to open camera!");
	}

	loop {
		if highgui::wait_key(10)? > 0 { break; }

		let mut frame = Mat::default();
		file.read(&mut frame)?;

        highgui::imshow(window, &frame)?;
	}

	Ok(())
}
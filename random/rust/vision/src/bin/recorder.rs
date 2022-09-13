//!

use opencv::{
	core::Size,
	highgui,
	prelude::*,
	Result,
	videoio,
};

fn main() -> Result<()> {
	let window = "video capture";
	highgui::named_window(window, 1)?;

    let mut cam = videoio::VideoCapture::new(2, videoio::CAP_ANY)?;

	if !videoio::VideoCapture::is_opened(&cam)? {
		panic!("Unable to open camera!");
	}

	let mut writer = videoio::VideoWriter::new(
			"./recording.mp4",
			videoio::VideoWriter::fourcc('x' as i8, 'v' as i8, 'i' as i8, 'd' as i8).unwrap(),
			cam.get(videoio::CAP_PROP_FPS).unwrap(),
			Size {
				width: cam.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap() as i32,
				height: cam.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap() as i32,
			},
			true)
		.unwrap();

	loop {
		if highgui::wait_key(10)? > 0 { break; }

		let mut frame = Mat::default();
		if !cam.read(&mut frame).expect("Failed to read next frame.") {
            break;
        }

        highgui::imshow(window, &frame)?;

		writer.write(&frame)?;
	}

	Ok(())
}
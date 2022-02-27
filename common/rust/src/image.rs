use std::convert::TryInto;
use std::path::PathBuf;

use nalgebra as na;
use image::{self, Pixel};

pub type ImageMatrix = na::OMatrix<f32, na::Dynamic, na::Dynamic>;

pub fn parse_raw_image(
    source_path: PathBuf
) -> ImageMatrix {
    let img = image::open(source_path)
        .unwrap()
        .to_luma8();

    let (width, height) = img.dimensions();

    let pixels = img
        .enumerate_pixels()
        .map(|(_, _, pixel)| (*pixel).channels()[0] as f32 / std::u8::MAX as f32)
        .collect::<Vec<f32>>();

    let matrix = na::DMatrix::from_row_slice(
        height.try_into().unwrap(),
        width.try_into().unwrap(),
        pixels.as_slice());

    matrix
}
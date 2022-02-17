use std::env;
use std::fs;
use std::io::{BufReader, BufRead};

use bytesize::ByteSize;
use nalgebra as na;

use malicious::kitti;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("Starting up...");
    log::warn!("Starting up...");
    log::debug!("Starting up...");
    log::error!("Starting up...");

    //let points = kitti::parse_raw_velodyne("/home/sushi/Datasets/kitti/raw/velodyne_points/data/0000000000.bin");
    //println!("{:?}", points);

    //let image = kitti::parse_raw_image("/home/sushi/Datasets/kitti/raw/image_00/data/0000000000.png");
    //println!("{:?}", image);

    //let tracklets = kitti::parse_raw_tracklets("/home/sushi/Datasets/kitti/raw/tracklet_labels.xml");
    //println!("{:?}", tracklets);

    //kitti::parse_raw_calib(
    //    "/home/sushi/Datasets/kitti/raw/calib_imu_to_velo.txt",
    //    "/home/sushi/Datasets/kitti/raw/calib_cam_to_cam.txt",
    //    "/home/sushi/Datasets/kitti/raw/calib_velo_to_cam.txt");

    //println!("{:?}", kitti::parse_raw_timestamps("/home/sushi/Datasets/kitti/raw/image_00/timestamps.txt"));

    //println!("{:?}", kitti::parse_raw_oxt("/home/sushi/Datasets/kitti/raw/oxts/data/0000000000.txt"));

    /*
    let raw_dataset = FrameParser::from(&[
        kitti::RawFile::Velodyne("/home/sushi/Datasets/kitti/raw/velodyne_points/data/0000000000.bin"),
        kitti::RawFile::Image(0, "/home/sushi/Datasets/kitti/raw/image_00/data/0000000000.png"),
        kitti::RawFile::Oxt("/home/sushi/Datasets/kitti/raw/oxts/data/0000000000.txt"),

        // TODO: should these be global?
        kitti::RawFile::Tracklets("/home/sushi/Datasets/kitti/raw/tracklet_labels.xml"),
        kitti::RawFile::Calib(
            "/home/sushi/Datasets/kitti/raw/calib_imu_to_velo.txt",
            "/home/sushi/Datasets/kitti/raw/calib_cam_to_cam.txt",
            "/home/sushi/Datasets/kitti/raw/calib_velo_to_cam.txt"),
    ]);

    println!("{:?}", raw_dataset);
    */

    //let total_points = kitti::parse_raw_velodyne_dir("/home/sushi/Datasets/kitti/raw/velodyne_points".to_string())
    //    .fold(0, |acc, x| acc + x.data.len());

    //println!("total points:      {}", total_points);
    //println!("approx mem needed: {}", ByteSize((total_points * std::mem::size_of::<na::Vector3<f32>>()) as u64));

    let images = kitti::parse_raw_images_dir("/home/sushi/Datasets/kitti/raw/image_00".to_string())
        .zip(kitti::parse_raw_images_dir("/home/sushi/Datasets/kitti/raw/image_01".to_string()))
        .zip(kitti::parse_raw_images_dir("/home/sushi/Datasets/kitti/raw/image_02".to_string()))
        .zip(kitti::parse_raw_images_dir("/home/sushi/Datasets/kitti/raw/image_03".to_string()));
    for (i, (((image_00, image_01), image_02), image_03)) in images.enumerate() {
        println!("{} {:?} {:?} {:?} {:?}", i, image_00, image_01, image_02, image_03);
    }
}
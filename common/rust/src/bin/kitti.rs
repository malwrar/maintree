use std::env;
use std::fs;
use std::io::{BufReader, BufRead};

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
}
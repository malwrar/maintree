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

    kitti::parse_raw("/home/sushi/Datasets/kitti/raw/".to_string())
        .for_each(|x| {
            println!("{:?}", x);
        })
}
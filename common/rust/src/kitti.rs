//! Tools for parsing the kitti visual odometry dataset available here:
//! http://www.cvlibs.net/datasets/kitti/eval_odometry.php
//!
//! Not really sure where to put this, but the dataset's authors request on their site that people
//! cite this somewhere:
//!
//!   @INPROCEEDINGS{Geiger2012CVPR,
//!   author = {Andreas Geiger and Philip Lenz and Raquel Urtasun},
//!   title = {Are we ready for Autonomous Driving? The KITTI Vision Benchmark Suite},
//!   booktitle = {Conference on Computer Vision and Pattern Recognition (CVPR)},
//!   year = {2012}

use chrono::prelude::*;

use std::cmp::Ordering;
use std::convert::TryInto;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use std::fs;
use std::io;

use nalgebra as na;
use regex::Regex;
use zip::read::{ZipArchive, ZipFile};

//lazy_static::lazy_static! {
    //// ex: dataset/sequences/21/velodyne/002552.bin
    //static ref ODOMETRY_VELODYNE_FNAME_META_EXTACTOR: Regex = Regex::new(r"(?x)
//dataset/sequences/
//(?P<sequence_num>\d+)/
//velodyne/
//(?P<capture_num>\d+)\.bin").unwrap();
//}

/////
//#[derive(Debug)]
//pub struct OdometryCapture {
    //pub sequence_num: u32,
    //pub capture_num: u32,
    //pub points: Vec<na::Vector3<f32>>,  // TODO: determine the coordinate system used by kitti
//}

///// The  total kitti dataset is over 100gb, so I wanted to ensure this extractor could work w/ a
///// partial copy. Multiple optional parameters seems to be best addressed with a builder, so here
///// it is.
//pub struct OdometryDatasetExtractorBuilder {
    //velodyne_archive: Option<ZipArchive<BufReader<File>>>,
//}

//impl OdometryDatasetExtractorBuilder {
    //pub fn velodyne_zip(mut self, filepath: &str) -> Self {
        //let file = File::open(filepath).expect("Unable to locate velodyne archive");
        //let file_reader = BufReader::new(file);

        //self.velodyne_archive = match ZipArchive::new(file_reader) {
            //Ok(a) => Some(a),
            //_ => {
                //log::warn!("Couldn't open provided velodyne archive @ {}",
                    //filepath);
                //None
            //}
        //};

        //self
    //}

    //pub fn build(self) -> OdometryDatasetExtractor {
        //OdometryDatasetExtractor {
            //velodyne_archive: self.velodyne_archive.expect("Couldn't find velodyne file!"),
        //}
    //}
//}

//pub struct OdometryDatasetExtractor {
    //velodyne_archive: ZipArchive<BufReader<File>>,
//}

//impl OdometryDatasetExtractor {
    //pub fn construct() -> OdometryDatasetExtractorBuilder {
        //OdometryDatasetExtractorBuilder {
            //velodyne_archive: None,            
        //}
    //}

    ///// Iterates over every Velodyne capture available.
    /////
    ///// This function can thus be thought of as a fairly rough data-read primitive,
    ///// likely requiring additional work to acheive more specific goals. To make
    ///// this easy, this function makes a few garuntees:
    /////
    /////   1.) All captures for the current sequence will be yielded before captures from any other
    /////       sequence.
    /////   2.) Captures will be yielded in ascending order by their capture number.
    /////
    ///// This effectively means that the data returned is sorted, allowing for a larger set of
    ///// processing operations to take place (like `filter()`ing).
    /////
    ///// Each capture likely will be quite large in size, so be cautious around `collect()`ing these
    ///// data into an in-memory datastructure like `Vec`!
    //pub fn captures(&mut self) -> impl Iterator<Item = OdometryCapture> + '_ {
        //// Sort metadata
        //let mut metadata: Vec<(u32, u32, usize)> = (0..self.velodyne_archive.len())
            //.filter_map(|idx| {
                //let file = self.velodyne_archive
                    //.by_index(idx)
                    //.expect("Failed to look up file.");
                //let filename = file.name();

                //let cap = match ODOMETRY_VELODYNE_FNAME_META_EXTACTOR.captures(filename) {
                    //Some(cap) => cap,
                    //None => return None,
                //};
                //let file_index = idx.clone();
                //let sequence_num = &cap["sequence_num"].parse::<u32>().unwrap();
                //let capture_num = &cap["capture_num"].parse::<u32>().unwrap();

                //Some((*sequence_num, *capture_num, file_index))
            //})
            //.collect();
        //log::debug!("Parsed velodyne archive metadata.");

        //metadata.sort_by(|a, b| {
            //let ord = a.0.cmp(&b.0);
            //if ord != Ordering::Equal {
                //return ord;
            //}

            //a.1.cmp(&b.1)
        //});
        //log::debug!("Sorted velodyne archive metadata.");

        //metadata.into_iter()
            //.map(move |(sequence_num, capture_num, archive_file_index)| {
                //let mut file = self.velodyne_archive
                    //.by_index(archive_file_index)
                    //.unwrap();

                //let mut points = Vec::new();
                //loop {
                    //let mut chunk = [0u8; 16];
                    //if file.read(&mut chunk).unwrap() < 16 {
                        //break;
                    //}

                    //let x = f32::from_ne_bytes(chunk[0..4].try_into().unwrap());
                    //let y = f32::from_ne_bytes(chunk[4..8].try_into().unwrap());
                    //let z = f32::from_ne_bytes(chunk[8..12].try_into().unwrap());
                    //let _ = f32::from_ne_bytes(chunk[12..16].try_into().unwrap());

                    //points.push(na::Vector3::new(x, y, z));
                //}

                //OdometryCapture {
                    //sequence_num,
                    //capture_num,
                    //points,
                //}
            //})
    //}
//}

#[derive(Debug)]
pub struct RawDatasetFrame {
    pub timestamp: DateTime<Utc>,
    pub image: PathBuf,
}

pub struct RawDatasetExtractor {
    timestamps: Vec<DateTime<Utc>>,
    images: Vec<PathBuf>,
    points: Vec<PathBuf>,
}

impl RawDatasetExtractor {
    pub fn new(base_path: PathBuf)-> Self {
        let mut timestamps_path = base_path.clone();
        timestamps_path.push("image_00/timestamps.txt");

        let f = fs::File::open(timestamps_path).unwrap();
        let timestamps = BufReader::new(f)
            .lines()
            .filter_map(|line| {
                match line {
                    Ok(line) => Some(line),
                    _ => None,
                }
            })
            .map(|line| Utc.datetime_from_str(line.as_str(), "%Y-%m-%d %H:%M:%S%.f").unwrap())
            .collect();

        let mut images_dir = base_path.clone();
        images_dir.push("image_00/data");

        let images = fs::read_dir(images_dir).unwrap()
            .map(|res| res.map(|e| e.path()).unwrap())
            .collect();

        let mut points_dir = base_path.clone();
        points_dir.push("velodyne_points/data");

        let points = fs::read_dir(points_dir).unwrap()
            .map(|res| res.map(|e| e.path()).unwrap())
            .collect();

        Self {
            timestamps,
            images,
            points,
        }
    }

    pub fn frames(&self) -> impl Iterator<Item=RawDatasetFrame> + '_ {
        self.timestamps.iter()
            .zip(self.images.iter())
            .map(|(timestamp, image)| RawDatasetFrame { 
                timestamp: timestamp.clone(),
                image: image.clone(),
            })
    }
}
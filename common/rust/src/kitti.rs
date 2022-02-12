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

use std::cmp::Ordering;
use std::convert::{TryInto, AsRef};
use std::io::{self, Read, BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::fs;

use chrono::prelude::*;

use image::{self, GenericImageView, Pixel};
use nalgebra as na;
use quick_xml;
use regex::Regex;
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
pub struct Pose {
    tx: f64,
    ty: f64,
    tz: f64,
    rx: f64,
    ry: f64,
    rz: f64,
    state: i32,
    occlusion: f64,
    occlusion_kf: f64,
    truncation: f64,
    amt_occlusion: f64,
    amt_occlusion_kf: f64,
    amt_border_l: f64,
    amt_border_r: f64,
    amt_border_kf: f64,
}

#[derive(Debug, Deserialize)]
pub struct Poses {
    count: usize,
    item: Vec<Pose>,
}

#[derive(Debug, Deserialize)]
pub struct Object {
    objectType: String,
    h: f64,
    w: f64,
    l: f64,
    first_frame: usize,
    poses: Poses,
}

#[derive(Debug, Deserialize)]
pub struct Tracklets {
    count: usize,
    item: Vec<Object>,
}

#[derive(Debug, Deserialize)]
pub struct Document {
    tracklets: Tracklets,
}

pub fn parse_raw_tracklets<S: Into<String> + AsRef<Path>>(source: S) -> Document {
    let mut tracklet_data = String::new();
    let f = fs::File::open(source)
        .unwrap()
        .read_to_string(&mut tracklet_data)
        .unwrap();
        
    quick_xml::de::from_str(&tracklet_data).unwrap()
}

pub fn parse_raw_image<S: Into<String> + AsRef<Path>>(source: S) -> na::OMatrix<f32, na::Dynamic, na::Dynamic> {
    let img = image::open(source)
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

pub fn parse_raw_velodyne<S: Into<String> + AsRef<Path>>(source: S) -> Vec<na::Vector3<f32>> {
    let mut f = fs::File::open(source).unwrap();
    parse_raw_velodyne_buf(&mut f)
}

pub fn parse_raw_velodyne_buf<T: io::Read>(source: &mut T) -> Vec<na::Vector3<f32>> {
    let mut points = Vec::new();

    loop {
        let mut chunk = [0u8; 16];
        if source.read(&mut chunk).unwrap() < 16 {
            break;
        }

        let x = f32::from_ne_bytes(chunk[0..4].try_into().unwrap());
        let y = f32::from_ne_bytes(chunk[4..8].try_into().unwrap());
        let z = f32::from_ne_bytes(chunk[8..12].try_into().unwrap());
        let _ = f32::from_ne_bytes(chunk[12..16].try_into().unwrap());

        points.push(na::Vector3::new(x, y, z));
    }

    points
}

fn parse_raw_calib_file_pairs<S: Into<String> + AsRef<Path>>(
    source: S
) -> (DateTime<Utc>, Vec<(String, Vec<f64>)>) {
    let mut timestamp = Utc::now();

    let f = fs::File::open(source).unwrap();
    let data = BufReader::new(f)
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let mut items = line.split(' ');

            let label = match items.next() {
                Some(label) => String::from(label),
                None => panic!("Failed to parse calib label."),
            };
            let label = String::from(label.trim_end_matches(":"));

            if label == "calib_time" {
                let timestamp_ = items
                    .fold(String::new(), |acc, next| acc + " " + next);

                timestamp = Utc
                    .datetime_from_str(timestamp_.trim(), "%v %T")
                    .unwrap();

                return None;
            }

            let data = items
                .flat_map(str::parse::<f64>)
                .collect::<Vec<f64>>();

            Some((label, data))
        })
        .collect::<Vec<(String, Vec<f64>)>>();

    (timestamp, data)
}

pub struct CalibrationParams {

}

pub fn parse_raw_calib<S: Into<String> + AsRef<Path>>(
    imu_to_velo_path: S,
    cam_to_cam_path: S,
    velo_to_cam_path: S,
) -> CalibrationParams {
    let mut pairs = Vec::new();
    pairs.extend(parse_raw_calib_file_pairs(imu_to_velo_path).1);
    pairs.extend(parse_raw_calib_file_pairs(cam_to_cam_path).1);
    pairs.extend(parse_raw_calib_file_pairs(velo_to_cam_path).1);

    // TODO: do something with this data
    println!("{:?}", pairs);

    CalibrationParams {}
}

pub fn parse_raw_timestamps<S: Into<String> + AsRef<Path>>(
    timestamps_path: S,
) -> Vec<DateTime<Utc>> {
    let f = fs::File::open(timestamps_path).unwrap();
    BufReader::new(f)
        .lines()
        .map(|line| Utc.datetime_from_str(&line.unwrap(), "%F %T%.f").unwrap())
        .collect::<Vec<DateTime<Utc>>>()
}

#[derive(Debug)]
pub struct Oxt {
    lat: f64,  // latitude of the oxts-unit (deg)
    lon: f64,  // longitude of the oxts-unit (deg)
    alt: f64,  // altitude of the oxts-unit (m)
    roll: f64,  // roll angle (rad),    0 = level, positive = left side up,      range: -pi   .. +pi
    pitch: f64,  // pitch angle (rad),   0 = level, positive = front down,        range: -pi/2 .. +pi/2
    yaw: f64,  // heading (rad),       0 = east,  positive = counter clockwise, range: -pi   .. +pi
    vn: f64,  // velocity towards north (m/s)
    ve: f64,  // velocity towards east (m/s)
    vf: f64,  // forward velocity, i.e. parallel to earth-surface (m/s)
    vl: f64,  // leftward velocity, i.e. parallel to earth-surface (m/s)
    vu: f64,  // upward velocity, i.e. perpendicular to earth-surface (m/s)
    ax: f64,  // acceleration in x, i.e. in direction of vehicle front (m/s^2)
    ay: f64,  // acceleration in y, i.e. in direction of vehicle left (m/s^2)
    az: f64,  // acceleration in z, i.e. in direction of vehicle top (m/s^2)
    af: f64,  // forward acceleration (m/s^2)
    al: f64,  // leftward acceleration (m/s^2)
    au: f64,  // upward acceleration (m/s^2)
    wx: f64,  // angular rate around x (rad/s)
    wy: f64,  // angular rate around y (rad/s)
    wz: f64,  // angular rate around z (rad/s)
    wf: f64,  // angular rate around forward axis (rad/s)
    wl: f64,  // angular rate around leftward axis (rad/s)
    wu: f64,  // angular rate around upward axis (rad/s)
    pos_accuracy: f64,  // velocity accuracy (north/east in m)
    vel_accuracy: f64,  // velocity accuracy (north/east in m/s)
    navstat: f64,  // navigation status (see navstat_to_string)
    numsats: f64,  // number of satellites tracked by primary GPS receiver
    posmode: f64,  // position mode of primary GPS receiver (see gps_mode_to_string)
    velmode: f64,  // velocity mode of primary GPS receiver (see gps_mode_to_string)
    orimode: f64,  // orientation mode of primary GPS receiver (see gps_mode_to_string)
}

pub fn parse_raw_oxt<S: Into<String> + AsRef<Path>>(
    oxt_path: S,
) -> Oxt {
    let f = fs::File::open(oxt_path).unwrap();
    BufReader::new(f)
        .lines()
        .map(|line| {
            let floats = line
                .unwrap()
                .split(' ')
                .flat_map(str::parse::<f64>)
                .collect::<Vec<f64>>();

            if floats.len() != 30 {
                panic!("Bad number of oxt datapoints in frame, failed to parse.");
            }

            // Gag...
            Oxt {
                lat: floats[0],
                lon: floats[1],
                alt: floats[2],
                roll: floats[3],
                pitch: floats[4],
                yaw: floats[5],
                vn: floats[6],
                ve: floats[7],
                vf: floats[8],
                vl: floats[9],
                vu: floats[10],
                ax: floats[11],
                ay: floats[12],
                az: floats[13],
                af: floats[14],
                al: floats[15],
                au: floats[16],
                wx: floats[17],
                wy: floats[18],
                wz: floats[19],
                wf: floats[20],
                wl: floats[21],
                wu: floats[22],
                pos_accuracy: floats[23],
                vel_accuracy: floats[24],
                navstat: floats[25],
                numsats: floats[26],
                posmode: floats[27],
                velmode: floats[28],
                orimode: floats[29],
            }
        })
        .last()  // There really should only be one
        .unwrap()
}

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
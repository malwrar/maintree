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

use std::collections::{BinaryHeap, HashMap};
use std::convert::{TryInto, AsRef};
use std::io::{self, Read, BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::fs;

use chrono::prelude::*;

use image::{self, Pixel};
use nalgebra as na;
use quick_xml;
use serde::Deserialize;

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

pub fn parse_raw_tracklets(source_path: PathBuf) -> Document {
    let mut tracklet_data = String::new();
    let f = fs::File::open(source_path)
        .unwrap()
        .read_to_string(&mut tracklet_data)
        .unwrap();
        
    quick_xml::de::from_str(&tracklet_data).unwrap()
}

type ImageMatrix = na::OMatrix<f32, na::Dynamic, na::Dynamic>;

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

pub fn parse_raw_velodyne(
    source_path: PathBuf
) -> Vec<na::Vector3<f32>> {
    let mut f = fs::File::open(source_path).unwrap();
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

fn parse_raw_calib_file_pairs(
    calib_path: PathBuf
) -> (DateTime<Utc>, Vec<(String, Vec<f64>)>) {
    let mut timestamp = Utc::now();

    let f = fs::File::open(calib_path).unwrap();
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

#[derive(Debug)]
pub struct ImageFrame {
    pub timestamp: DateTime<Utc>,
    pub image:     ImageMatrix,
}

pub fn parse_raw_images_dir<'a>(
    base_path: PathBuf,
) -> impl Iterator<Item = ImageFrame> {
    let timestamps = parse_raw_timestamps(base_path.join("timestamps.txt"));

    // Parse data and insert it into the currently empty `data` vector.
    let mut map = fs::read_dir(base_path.join("data"))
        .unwrap()
        .zip(timestamps)
        .map(|(path, timestamp)| {
            let path = path
                .unwrap()
                .path();

            let idx = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<usize>().unwrap();

            let path = path
                .to_str()
                .unwrap();

            (idx, timestamp, PathBuf::from(path))
        })
        .collect::<Vec<(usize, DateTime<Utc>, PathBuf)>>();

    map.sort_by(|a, b| a.0.cmp(&b.0));

    map
        .into_iter()
        .map(|(_, timestamp, path)| {
            ImageFrame {
                timestamp,
                image: parse_raw_image(path),
            }
        })
}

#[derive(Debug)]
pub struct VelodyneFrame {
    pub timestamp: DateTime<Utc>,
    pub start:     DateTime<Utc>,
    pub end:       DateTime<Utc>,
    pub data:      Vec<na::Vector3<f32>>,
}

pub fn parse_raw_velodyne_dir(
    base_path: PathBuf,
) -> impl Iterator<Item = VelodyneFrame> {
    // Initiate `captures` with timestamps.
    let timestamps_start = parse_raw_timestamps(base_path.join("timestamps_start.txt"));
    let timestamps_end   = parse_raw_timestamps(base_path.join("timestamps_end.txt"));
    let timestamps       = parse_raw_timestamps(base_path.join("timestamps.txt"));

    // Parse data and insert it into the currently empty `data` vector.
    let mut map = fs::read_dir(base_path.join("data"))
        .unwrap()
        .zip(timestamps)
        .zip(timestamps_start)
        .zip(timestamps_end)
        .map(|(((path, timestamp), start), end)| {
            let path = path
                .unwrap()
                .path();

            let idx = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<usize>().unwrap();

            let path = path
                .to_str()
                .unwrap();

            (idx, timestamp, start, end, PathBuf::from(path))
        })
        .collect::<Vec<(usize, DateTime<Utc>, DateTime<Utc>, DateTime<Utc>, PathBuf)>>();

    map.sort_by(|a, b| a.0.cmp(&b.0));

    map
        .into_iter()
        .map(|(_, timestamp, start, end, path)| {
            VelodyneFrame {
                timestamp,
                start,
                end,
                data: parse_raw_velodyne(path)
            }
        })
}

pub struct CalibrationParams {

}

pub fn parse_raw_calib(
    imu_to_velo_path: PathBuf,
    cam_to_cam_path: PathBuf,
    velo_to_cam_path: PathBuf,
) -> CalibrationParams {
    let mut pairs = Vec::new();
    pairs.extend(parse_raw_calib_file_pairs(imu_to_velo_path).1);
    pairs.extend(parse_raw_calib_file_pairs(cam_to_cam_path).1);
    pairs.extend(parse_raw_calib_file_pairs(velo_to_cam_path).1);

    // TODO: do something with this data
    println!("{:?}", pairs);

    CalibrationParams {}
}

pub fn parse_raw_timestamps(
    timestamps_path: PathBuf,
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

pub fn parse_raw_oxt(oxt_path: PathBuf) -> Oxt {
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
pub struct OxtFrame {
    pub timestamp: DateTime<Utc>,
    pub oxt:       Oxt,
}

pub fn parse_raw_oxt_dir(
    base_path: PathBuf,
) -> impl Iterator<Item = OxtFrame> {
    let timestamps = parse_raw_timestamps(base_path.join("timestamps.txt"));

    // Parse data and insert it into the currently empty `data` vector.
    let mut map = fs::read_dir(base_path.join("data"))
        .unwrap()
        .zip(timestamps)
        .map(|(path, timestamp)| {
            let path = path
                .unwrap()
                .path();

            let idx = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<usize>().unwrap();

            let path = path
                .to_str()
                .unwrap();

            (idx, timestamp, PathBuf::from(path))
        })
        .collect::<Vec<(usize, DateTime<Utc>, PathBuf)>>();

    map.sort_by(|a, b| a.0.cmp(&b.0));

    map
        .into_iter()
        .map(|(_, timestamp, path)| {
            OxtFrame {
                timestamp,
                oxt: parse_raw_oxt(path),
            }
        })
}

#[derive(Debug)]
pub struct Frame {
    oxt: OxtFrame,
    velodyne: VelodyneFrame,
    image_00: ImageFrame,
    image_01: ImageFrame,
    image_02: ImageFrame,
    image_03: ImageFrame,
}

pub fn parse_raw(
    base_path: String,
) -> impl Iterator<Item = Frame> {
    let base_path = Path::new(&base_path);
    parse_raw_oxt_dir(base_path.join("oxts/"))
        .zip(parse_raw_velodyne_dir(base_path.join("velodyne_points/")))
        .zip(parse_raw_images_dir(base_path.join("image_00/")))
        .zip(parse_raw_images_dir(base_path.join("image_01/")))
        .zip(parse_raw_images_dir(base_path.join("image_02/")))
        .zip(parse_raw_images_dir(base_path.join("image_03/")))
        .map(|(((((oxt, velodyne), image_00), image_01), image_02), image_03)| {
            Frame {
                oxt,
                velodyne,
                image_00,
                image_01,
                image_02,
                image_03,
            }
        })
}
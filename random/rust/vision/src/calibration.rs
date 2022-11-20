//! Tools for calibrating cameras.

use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use opencv::{
	prelude::*,
	core::{
		Point2f,
		Point3f,
		Size,
		TermCriteria,
		TermCriteria_Type,
        Vector,
        CV_64F,
	},
	calib3d,
	Result,
	types::{
		VectorOfVectorOfPoint3f,
		VectorOfVectorOfPoint2f,
	},
};

use serde::{Serialize, Deserialize};

use crate::mapping::MetricMap;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct CameraCalibration {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub p1: f64,
    pub p2: f64,
}

impl CameraCalibration {
    pub fn from_observations(
        capture_width: i32,
        capture_height: i32,
        object_points: Vec<Vec<Point3f>>,
        image_points: Vec<Vec<Point2f>>,
        rvecs: &mut Mat,
        tvecs: &mut Mat,
    ) -> Result<(Self, MetricMap)> {
	    let mut k = Mat::default();
	    let mut dist_coeffs = Mat::default();

        calib3d::calibrate_camera(
                &VectorOfVectorOfPoint3f::from_iter(
                    object_points
                        .iter()
                        .map(|v| {
                            Vector::from_iter(v
                                .iter()
                                .map(|pt| *pt))
                        })
                ),
                &VectorOfVectorOfPoint2f::from_iter(
                    image_points
                        .iter()
                        .map(|v| {
                            Vector::from_iter(v
                                .iter()
                                .map(|pt| *pt))
                        })
                ),
                Size::new(capture_width, capture_height),
                &mut k,
                &mut dist_coeffs,
                rvecs,
                tvecs,
                0,
                TermCriteria::new(
	    		        TermCriteria_Type::EPS as i32
                            | TermCriteria_Type::COUNT as i32,
	    			    30,
                        std::f64::EPSILON)
                    .unwrap()
            )
            .unwrap();

        // TODO: create metric map from observation of known pattern
        let map = MetricMap::new();

        let calibration = Self {
            fx: *k.at_2d_mut(0, 0).unwrap(),
            fy: *k.at_2d_mut(1, 1).unwrap(),
            cx: *k.at_2d_mut(0, 2).unwrap(),
            cy: *k.at_2d_mut(1, 2).unwrap(),
            k1: *dist_coeffs.at_2d_mut(0, 0).unwrap(),
            k2: *dist_coeffs.at_2d_mut(0, 1).unwrap(),
            p1: *dist_coeffs.at_2d_mut(0, 2).unwrap(),
            p2: *dist_coeffs.at_2d_mut(0, 3).unwrap(),
            k3: *dist_coeffs.at_2d_mut(0, 4).unwrap(),
        };

        //assert!(calibration.k(). == k);
        //assert!(calibration.dist_coeffs() == dist_coeffs);

        Ok((calibration, map))
    }

    pub fn from_file(path: String) -> std::io::Result<Self> {
        let mut reader = BufReader::new(File::open(path)?);
        let mut contents = String::new();

        reader.read_to_string(&mut contents)?;

        let calib = serde_yaml::from_str(&contents)
            .expect("Failedt to parse calib file!");

        Ok(calib)
    }

    pub fn write_to_file(&self, path: String) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        file.write_all(serde_yaml::to_string(&self).unwrap().as_bytes())?;

        Ok(())
    }

    pub fn k(&self) -> Mat {
	    let mut k = Mat::eye_size(Size::new(3, 3), CV_64F)
            .unwrap()
            .to_mat()
            .unwrap();

        *k.at_2d_mut(0, 0).unwrap() = self.fx;
        *k.at_2d_mut(1, 1).unwrap() = self.fy;
        *k.at_2d_mut(0, 2).unwrap() = self.cx;
        *k.at_2d_mut(1, 2).unwrap() = self.cy;

        k
    }

    pub fn dist_coeffs(&self) -> Mat {
	    let mut dist_coeffs = Mat::zeros(1, 5, CV_64F)
            .unwrap()
            .to_mat()
            .unwrap();

        *dist_coeffs.at_2d_mut(0, 0).unwrap() = self.k1;
        *dist_coeffs.at_2d_mut(0, 1).unwrap() = self.k2;
        *dist_coeffs.at_2d_mut(0, 2).unwrap() = self.p1;
        *dist_coeffs.at_2d_mut(0, 3).unwrap() = self.p2;
        *dist_coeffs.at_2d_mut(0, 4).unwrap() = self.k3;

        dist_coeffs
    }

    /// Generate an opengl projection matrix from intrinsic camera params.
    /// 
    /// Based on https://strawlab.org/2011/11/05/augmented-reality-with-OpenGL/
    fn opengl_projection_matrix(
        &self,
        width: f64,
        height: f64,
        x0: f64,
        y0: f64,
        znear: f64,
        zfar: f64,
    ) -> Mat {
        // here be dragons
        Mat::from_slice_2d(&[
            [2.0 * self.fx / width,  0.0, (width - 2.0 * self.cx + 2.0 * x0) / width,  0.0],
            [0.0, -2.0 * self.fy / height, (height - 2.0 * self.cy + 2.0 * y0) / height,  0.0],
            [0.0,  0.0, (-zfar - znear) / (zfar - znear), -2.0 * zfar * znear / (zfar - znear)],
            [0.0,  0.0, -1.0,  0.0],
        ])
        .unwrap()
    }
}

impl Default for CameraCalibration {
    fn default() -> Self {
        Self {
            fx: 1.0,
            fy: 1.0,
            cx: 0.0,
            cy: 0.0,
            k1: 0.0,
            k2: 0.0,
            p1: 0.0,
            p2: 0.0,
            k3: 0.0,
        }
    }
}
//! Tool for realtime tracking of camera and environment state changes between
//! observations.

use crate::{
    mapping::{
        MetricMap,
    },
    calibration::CameraCalibration,
};

pub struct Tracker {
    calib: CameraCalibration,
    pub map: MetricMap,
}

impl Tracker {
    pub fn new(calib: CameraCalibration) -> Self {
        Self {
            calib,
            map: MetricMap::new(),
        }
    }
}
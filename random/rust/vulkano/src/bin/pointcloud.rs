extern crate bevy;
extern crate kdtree;
extern crate malicious;

use std::fs::File;

use kdtree::KdTree;
use kdtree::distance::squared_euclidean;
use bevy::{
    app::App,
    pbr::PbrBundle
    prelude::*,
};

use malicious::{
    parsing::kitti::VelodyneBinParser,
};

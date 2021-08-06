extern crate kdtree;

use std::fs::File;
use std::time::Instant;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

fn main() {
    env_logger::init();
    println!("Hello, world!");

    let mut cloud = KdTree::new(3);

    // Parse velodyne points.
    let t = Instant::now();

    let bin = File::open("/home/sushi/datasets/kitti/dataset/sequences/00/velodyne/000000.bin").unwrap();
    let points = parsing::kitti::VelodyneBinParser::new(bin);
    for point in points {
        cloud.add([point.0, point.1, point.2], 0);
    }

    log::info!("kitti velodyne points parsed in {:?} (total: {})", t.elapsed(),
            cloud.size());

    // Search velodyne points.
    let t = Instant::now();

    let num_points = 100;
    println!("{} points nearest center: {:?}",
             num_points,
             cloud.nearest(&[0f32, 0f32, 0f32], num_points, &squared_euclidean));

    log::info!("kitti velodyne searched in {:?}", t.elapsed());
}

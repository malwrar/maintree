extern crate parsing;

use std::fs::File;
use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let t = Instant::now();
    let bin = File::open("/home/sushi/datasets/kitti/dataset/sequences/00/velodyne/000000.bin").unwrap();
    let points = parsing::kitti::VelodyneBinParser::new(bin);
    for point in points {
        println!("point: {:?}", point);
    }
    println!("kitti velodyne points parsed in {:?}", t.elapsed());
}

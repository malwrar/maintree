use malicious::{
    parsing::kitti::KittiParser,
};

fn main() {
    let parser = KittiParser::new(&[
            "../../../../datasets/kitti/2011_09_29_calib.zip",
            //"../../../../datasets/kitti/2011_09_29_drive_0004_extract.zip",
            "../../../../datasets/kitti/2011_09_29_drive_0004_sync.zip",

        ]);
}

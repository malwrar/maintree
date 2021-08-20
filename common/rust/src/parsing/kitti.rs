//
extern crate regex;

use std::io::prelude::*;

use std::convert::TryInto;
use std::fs::File;

use regex::Regex;

// Some regex extractors for later.
lazy_static! {
    // ex: "2011_09_29/calib_cam_to_cam.txt"
    // ex: "2011_09_29/calib_imu_to_velo.txt"
    // ex: "2011_09_29/calib_velo_to_cam.txt"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/oxts/data/0000000437.txt"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/oxts/dataformat.txt"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/image_00/data/0000000657.png"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/image_01/data/0000000657.png"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/image_02/data/0000000657.png"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/image_03/data/0000000657.png"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/velodyne_points/data/0000000553.bin"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/velodyne_points/timestamps_start.txt"
    // ex: "2011_09_29/2011_09_29_drive_0071_sync/velodyne_points/timestamps_end.txt"
    static ref FILENAME_DATE_EXTRACTOR: Regex = Regex::new(r"(?x)
(?P<year>\d{4})
_
(?P<month>\d{2})
_
(?P<day>\d{2})  # neat compromise between american mm/dd/yyyy and sensible dd/mm/yyyy styles here!
/
(?P<nextdir>.+)").unwrap();
    static ref FILENAME_METADATA_EXTRACTOR: Regex = Regex::new(r"(?x)
[\d_/]_drive_
(?P<drive_number>\d{4})
_sync/
(?P<data_type>[^/]+)
/
(?P<data_path>.*)
").unwrap();
}

/// Kitti is big, so this interface is an easy way to parse the dataset straight from the zip files
/// it's distributed in. It doesn't even need to be a complete set either, just toss whatever zip
/// files you have and `KittiParser` will extract whatever is usable. 1
pub struct KittiParser {
}

impl KittiParser {
    pub fn new(dataset_zip_paths: &[&str]) -> Self {
        // We'll need this later, initialize now since it's cheaper
        for path in dataset_zip_paths {
            let zipfile = std::fs::File::open(&path).unwrap();
            let archive = zip::ZipArchive::new(zipfile).unwrap();

            for filename in archive.file_names() {
                // Since the kitti zip files are fairly structured (and probably not going to
                // change in structure) we can fulfill our "extract whatever is usable" contract
                // by just inferring datatype from metadata in the filepaths in the zip. It's not
                // like we can validate the data anyways (I bet you're thinking I could just check
                // hashes, in which case feel free to submit a PR implementing that!)
                let cap = match FILENAME_DATE_EXTRACTOR.captures(filename) {
                    Some(cap) => cap,
                    None => continue,
                };

                let year = &cap["year"];
                let month = &cap["month"];
                let day = &cap["day"];
                let nextdir = &cap["nextdir"];

                let cap = match FILENAME_METADATA_EXTRACTOR.captures(nextdir) {
                    Some(cap) => cap,
                    None => continue,
                };

                let drive_number = &cap["drive_number"];
                let data_type = &cap["data_type"];
                let data_path = &cap["data_path"];

                println!("{:?} {:?}", data_type, data_path);
            }
        }

        Self {
        }
    }
}

//impl Iterator for KittiParser {
//    type Item = (f32, f32, f32, f32);
//    
//    fn next(&mut self) -> Option<Self::Item> {
//        let mut chunk = [0; 16];
//        match self.buf.read(&mut chunk) {
//            Ok(0) => None,
//            Ok(n) => {
//                if n != 16 { panic!("Unaligned chunk of size {} found!", n) }
//
//                let x = f32::from_le_bytes(chunk[0..4].try_into().unwrap());
//                let y = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
//                let z = f32::from_le_bytes(chunk[8..12].try_into().unwrap());
//                let i = f32::from_le_bytes(chunk[12..16].try_into().unwrap());
//
//                Some((x, y, z, i))
//            }
//            Err(e) => panic!("{:?}", e),
//        }
//    }
//}

//pub struct AnnotationParser {
//    buf: BufReader<File>,
//}
//
//impl AnnotationParser {
//    pub fn new(file: File) -> Self {
//        let buf = BufReader::new(file);
//
//        Self {
//            buf,
//        }
//    }
//}
//
//pub struct VelodyneBinParser {
//    buf: BufReader<File>,
//}
//
//impl VelodyneBinParser {
//    pub fn new(file: File) -> Self {
//        let buf = BufReader::new(file);
//
//        Self {
//            buf,
//        }
//    }
//}
//
//impl Iterator for VelodyneBinParser {
//    type Item = (f32, f32, f32, f32);
//    
//    fn next(&mut self) -> Option<Self::Item> {
//        let mut chunk = [0; 16];
//        match self.buf.read(&mut chunk) {
//            Ok(0) => None,
//            Ok(n) => {
//                if n != 16 { panic!("Unaligned chunk of size {} found!", n) }
//
//                let x = f32::from_le_bytes(chunk[0..4].try_into().unwrap());
//                let y = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
//                let z = f32::from_le_bytes(chunk[8..12].try_into().unwrap());
//                let i = f32::from_le_bytes(chunk[12..16].try_into().unwrap());
//
//                Some((x, y, z, i))
//            }
//            Err(e) => panic!("{:?}", e),
//        }
//    }
//}

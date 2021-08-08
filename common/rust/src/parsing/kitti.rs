//! [kitti dataset](http://www.cvlibs.net/datasets/kitti/) parsing utilities.

use std::convert::TryInto;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};

pub struct VelodyneBinParser {
    buf: BufReader<File>,
}

impl VelodyneBinParser {
    pub fn new(file: File) -> Self {
        let buf = BufReader::new(file);

        Self {
            buf,
        }
    }
}

impl Iterator for VelodyneBinParser {
    type Item = (f32, f32, f32, f32);
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = [0; 16];
        match self.buf.read(&mut chunk) {
            Ok(0) => None,
            Ok(n) => {
                if n != 16 { panic!("Unaligned chunk of size {} found!", n) }

                let x = f32::from_le_bytes(chunk[0..4].try_into().unwrap());
                let y = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
                let z = f32::from_le_bytes(chunk[8..12].try_into().unwrap());
                let i = f32::from_le_bytes(chunk[12..16].try_into().unwrap());

                Some((x, y, z, i))
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}

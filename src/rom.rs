use std::io::prelude::*;
use std::fs::File;

#[derive(Clone, Copy)]
pub struct ROM {
    // 4096 (mem size) - unaddressable 512 slots
    pub data: [u8; 3584],
    pub size: usize, // True size
}

impl ROM {
    pub fn new(filename: &str) -> ROM {
        let mut f = File::open(filename).unwrap();

        let mut data = [0; 3584];

        let size = f.read(&mut data).unwrap();

        ROM {
            data,
            size,
        }
    }
}
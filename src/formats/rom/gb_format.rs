use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;


pub struct GbFormat;

impl Detector for GbFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        // check if data is long enough
        if data.len() < 0x14D {
            return Ok(false);
        }

        // Initialize x to 0
        let mut x: i32 = 0;

        // for each byte in range 0x134 to 0x14C, subtract the byte and 1 from x
        for &byte in &data[0x134..=0x14C] {
            x = x - (byte as i32) - 1;
        }

        // The checksum should equal the byte at 0x14D
        Ok((x & 0xFF) as u8 == data[0x14D])
    }
}

impl Validator for GbFormat {
}

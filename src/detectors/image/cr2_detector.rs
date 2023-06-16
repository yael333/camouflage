use crate::Detector;
use std::error::Error;
use infer;

pub struct Cr2Detector;

impl Detector for Cr2Detector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_cr2(data))
    }
}

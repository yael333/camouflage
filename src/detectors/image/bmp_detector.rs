use crate::Detector;
use std::error::Error;
use infer;

pub struct BmpDetector;

impl Detector for BmpDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_bmp(data))
    }
}

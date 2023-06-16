use crate::Detector;
use std::error::Error;
use infer;

pub struct AvifDetector;

impl Detector for AvifDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_avif(data))
    }
}

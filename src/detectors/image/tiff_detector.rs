use crate::Detector;
use std::error::Error;
use infer;

pub struct TiffDetector;

impl Detector for TiffDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_tiff(data))
    }
}

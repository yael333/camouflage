use crate::Detector;
use std::error::Error;
use infer;

pub struct Jpeg2000Detector;

impl Detector for Jpeg2000Detector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_jpeg2000(data))
    }
}

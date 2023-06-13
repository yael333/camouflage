use crate::Detector;
use std::error::Error;
use infer;

pub struct IcoDetector;

impl Detector for IcoDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_ico(data))
    }
}

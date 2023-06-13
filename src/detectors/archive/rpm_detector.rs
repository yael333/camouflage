use crate::Detector;
use std::error::Error;
use infer;

pub struct RpmDetector;

impl Detector for RpmDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_rpm(data))
    }
}

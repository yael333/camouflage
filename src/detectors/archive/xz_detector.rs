use crate::Detector;
use std::error::Error;
use infer;

pub struct XzDetector;

impl Detector for XzDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_xz(data))
    }
}

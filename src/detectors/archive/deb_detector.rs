use crate::Detector;
use std::error::Error;
use infer;

pub struct DebDetector;

impl Detector for DebDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_deb(data))
    }
}

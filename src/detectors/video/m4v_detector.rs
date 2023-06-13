use crate::Detector;
use std::error::Error;
use infer;

pub struct M4vDetector;

impl Detector for M4vDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_m4v(data))
    }
}

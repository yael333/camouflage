use crate::Detector;
use std::error::Error;
use infer;

pub struct WmvDetector;

impl Detector for WmvDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_wmv(data))
    }
}

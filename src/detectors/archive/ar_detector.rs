use crate::Detector;
use std::error::Error;
use infer;

pub struct ArDetector;

impl Detector for ArDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_ar(data))
    }
}

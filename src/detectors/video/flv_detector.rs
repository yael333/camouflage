use crate::Detector;
use std::error::Error;
use infer;

pub struct FlvDetector;

impl Detector for FlvDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_flv(data))
    }
}

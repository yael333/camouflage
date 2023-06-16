use crate::Detector;
use std::error::Error;
use infer;

pub struct MovDetector;

impl Detector for MovDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_mov(data))
    }
}
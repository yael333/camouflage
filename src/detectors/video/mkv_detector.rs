use crate::Detector;
use std::error::Error;
use infer;

pub struct MkvDetector;

impl Detector for MkvDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_mkv(data))
    }
}

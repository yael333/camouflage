use crate::Detector;
use std::error::Error;
use infer;

pub struct PptDetector;

impl Detector for PptDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_ppt(data))
    }
}

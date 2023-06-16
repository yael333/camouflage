use crate::Detector;
use std::error::Error;
use infer;

pub struct LzDetector;

impl Detector for LzDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_lz(data))
    }
}

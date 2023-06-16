use crate::Detector;
use std::error::Error;
use infer;

pub struct DsfDetector;

impl Detector for DsfDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_dsf(data))
    }
}

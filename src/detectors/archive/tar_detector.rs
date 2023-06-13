use crate::Detector;
use std::error::Error;
use infer;

pub struct TarDetector;

impl Detector for TarDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_tar(data))
    }
}

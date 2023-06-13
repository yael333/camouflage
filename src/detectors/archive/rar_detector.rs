use crate::Detector;
use std::error::Error;
use infer;

pub struct RarDetector;

impl Detector for RarDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_rar(data))
    }
}

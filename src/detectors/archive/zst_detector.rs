use crate::Detector;
use std::error::Error;
use infer;

pub struct ZstDetector;

impl Detector for ZstDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_zst(data))
    }
}

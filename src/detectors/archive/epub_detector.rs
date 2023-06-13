use crate::Detector;
use std::error::Error;
use infer;

pub struct EpubDetector;

impl Detector for EpubDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_epub(data))
    }
}

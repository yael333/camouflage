use crate::Detector;
use std::error::Error;
use infer;

pub struct Woff2Detector;

impl Detector for Woff2Detector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::font::is_woff2(data))
    }
}

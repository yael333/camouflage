use crate::Detector;
use std::error::Error;
use infer;

pub struct SwfDetector;

impl Detector for SwfDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_swf(data))
    }
}

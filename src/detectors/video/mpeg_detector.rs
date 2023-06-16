use crate::Detector;
use std::error::Error;
use infer;

pub struct MpegDetector;

impl Detector for MpegDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_mpeg(data))
    }
}

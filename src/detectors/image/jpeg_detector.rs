use crate::Detector;
use std::error::Error;
use infer;

pub struct JpegDetector;

impl Detector for JpegDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_jpeg(data))
    }
}

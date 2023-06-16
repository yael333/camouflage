use crate::Detector;
use std::error::Error;
use infer;

pub struct JxlDetector;

impl Detector for JxlDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_jxl(data))
    }
}

use crate::Detector;
use std::error::Error;
use infer;

pub struct JxrDetector;

impl Detector for JxrDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_jxr(data))
    }
}

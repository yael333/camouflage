use crate::Detector;
use std::error::Error;
use infer;

pub struct XmlDetector;

impl Detector for XmlDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::text::is_xml(data))
    }
}

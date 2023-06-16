use crate::Detector;
use std::error::Error;
use infer;

pub struct HtmlDetector;

impl Detector for HtmlDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::text::is_html(data))
    }
}

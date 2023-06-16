use crate::Detector;
use std::error::Error;
use infer;

pub struct WebpDetector;

impl Detector for WebpDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_webp(data))
    }
}

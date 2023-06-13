use crate::Detector;
use std::error::Error;
use infer;

pub struct GifDetector;

impl Detector for GifDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_gif(data))
    }
}

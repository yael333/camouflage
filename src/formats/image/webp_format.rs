use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct WebpFormat;

impl Detector for WebpFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_webp(data))
    }
}
impl Validator for WebpFormat {
}

use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct WebmFormat;

impl Detector for WebmFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_webm(data))
    }
}
impl Validator for WebmFormat {
}

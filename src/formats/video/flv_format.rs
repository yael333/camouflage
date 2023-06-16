use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct FlvFormat;

impl Detector for FlvFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_flv(data))
    }
}
impl Validator for FlvFormat {
}

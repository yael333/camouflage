use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct WmvFormat;

impl Detector for WmvFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_wmv(data))
    }
}
impl Validator for WmvFormat {
}

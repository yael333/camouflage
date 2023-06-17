use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct JxrFormat;

impl Detector for JxrFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_jxr(data))
    }
}
impl Validator for JxrFormat {
}

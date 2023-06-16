use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct JxlFormat;

impl Detector for JxlFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_jxl(data))
    }
}
impl Validator for JxlFormat {
}

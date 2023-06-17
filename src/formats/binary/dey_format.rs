use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DeyFormat;

impl Detector for DeyFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_dey(data))
    }
}
impl Validator for DeyFormat {
}

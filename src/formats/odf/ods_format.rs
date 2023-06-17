use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct OdsFormat;

impl Detector for OdsFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::odf::is_ods(data))
    }
}
impl Validator for OdsFormat {
}
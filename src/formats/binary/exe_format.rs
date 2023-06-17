use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct ExeFormat;

impl Detector for ExeFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_exe(data))
    }
}
impl Validator for ExeFormat {
}

use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct PptFormat;

impl Detector for PptFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_ppt(data))
    }
}
impl Validator for PptFormat {
}

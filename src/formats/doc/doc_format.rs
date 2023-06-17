use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DocFormat;

impl Detector for DocFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_doc(data))
    }
}
impl Validator for DocFormat {
}

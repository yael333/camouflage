use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DebFormat;

impl Detector for DebFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_deb(data))
    }
}
impl Validator for DebFormat {
}

use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct SwfFormat;

impl Detector for SwfFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_swf(data))
    }
}
impl Validator for SwfFormat {
}

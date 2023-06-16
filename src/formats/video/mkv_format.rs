use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct MkvFormat;

impl Detector for MkvFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_mkv(data))
    }
}
impl Validator for MkvFormat {
}

use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct DsfFormat;

impl Detector for DsfFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_dsf(data))
    }
}
impl Validator for DsfFormat {
}

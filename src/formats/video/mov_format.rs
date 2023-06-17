use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct MovFormat;

impl Detector for MovFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_mov(data))
    }
}impl Validator for MovFormat {
}

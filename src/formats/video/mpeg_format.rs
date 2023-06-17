use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct MpegFormat;

impl Detector for MpegFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_mpeg(data))
    }
}
impl Validator for MpegFormat {
}

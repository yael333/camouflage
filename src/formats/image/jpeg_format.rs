use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct JpegFormat;

impl Detector for JpegFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_jpeg(data))
    }
}
impl Validator for JpegFormat {
}

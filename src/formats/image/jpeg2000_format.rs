use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Jpeg2000Format;

impl Detector for Jpeg2000Format {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_jpeg2000(data))
    }
}
impl Validator for Jpeg2000Format {
}

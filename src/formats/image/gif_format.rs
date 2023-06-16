use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct GifFormat;

impl Detector for GifFormat {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_gif(data))
    }
}
impl Validator for GifFormat {
}

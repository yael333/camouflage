use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct Ogg_opusFormat;

impl Detector for Ogg_opusFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_ogg_opus(data))
    }
}
impl Validator for Ogg_opusFormat {
}

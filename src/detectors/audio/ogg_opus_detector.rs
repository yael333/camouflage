use crate::Detector;
use std::error::Error;
use infer;

pub struct Ogg_opusDetector;

impl Detector for Ogg_opusDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_ogg_opus(data))
    }
}

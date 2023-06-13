use crate::Detector;
use std::error::Error;
use infer;

pub struct Mp3Detector;

impl Detector for Mp3Detector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_mp3(data))
    }
}

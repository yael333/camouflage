use crate::Detector;
use std::error::Error;
use infer;

pub struct MidiDetector;

impl Detector for MidiDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_midi(data))
    }
}

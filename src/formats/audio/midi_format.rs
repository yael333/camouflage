use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct MidiFormat;

impl Detector for MidiFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_midi(data))
    }
}
impl Validator for MidiFormat {
}

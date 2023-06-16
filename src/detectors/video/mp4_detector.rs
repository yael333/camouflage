use crate::Detector;
use std::error::Error;
use infer;

pub struct Mp4Detector;

impl Detector for Mp4Detector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::video::is_mp4(data))
    }
}

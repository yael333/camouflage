use crate::Detector;
use std::error::Error;
use infer;

pub struct Bz2Detector;

impl Detector for Bz2Detector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_bz2(data))
    }
}

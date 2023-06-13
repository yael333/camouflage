use crate::Detector;
use std::error::Error;
use infer;

pub struct WasmDetector;

impl Detector for WasmDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_wasm(data))
    }
}

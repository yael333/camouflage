mod png_detector;
mod jpeg_detector;

pub use self::png_detector::PngDetector;
pub use self::jpeg_detector::JpegDetector;

use std::error::Error;

pub trait Detector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>>;
}

mod otf_detector;
mod ttf_detector;
mod woff_detector;
mod woff2_detector;

pub use self::otf_detector::OtfDetector;
pub use self::ttf_detector::TtfDetector;
pub use self::woff_detector::WoffDetector;
pub use self::woff2_detector::Woff2Detector;
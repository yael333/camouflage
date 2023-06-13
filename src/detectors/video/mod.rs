mod avi_detector;
mod flv_detector;
mod m4v_detector;
mod mkv_detector;
mod mov_detector;
mod mp4_detector;
mod mpeg_detector;
mod webm_detector;
mod wmv_detector;

pub use self::avi_detector::AviDetector;
pub use self::flv_detector::FlvDetector;
pub use self::m4v_detector::M4vDetector;
pub use self::mkv_detector::MkvDetector;
pub use self::mov_detector::MovDetector;
pub use self::mp4_detector::Mp4Detector;
pub use self::mpeg_detector::MpegDetector;
pub use self::webm_detector::WebmDetector;
pub use self::wmv_detector::WmvDetector;
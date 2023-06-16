mod aac_format;
mod aiff_format;
mod amr_format;
mod ape_format;
mod dsf_format;
mod flac_format;
mod m4a_format;
mod midi_format;
mod mp3_format;
mod ogg_format;
mod ogg_opus_format;
mod wav_format;

pub use self::aac_format::AacFormat;
pub use self::aiff_format::AiffFormat;
pub use self::amr_format::AmrFormat;
pub use self::ape_format::ApeFormat;
pub use self::dsf_format::DsfFormat;
pub use self::flac_format::FlacFormat;
pub use self::m4a_format::M4aFormat;
pub use self::midi_format::MidiFormat;
pub use self::mp3_format::Mp3Format;
pub use self::ogg_format::OggFormat;
pub use self::ogg_opus_format::Ogg_opusFormat;
pub use self::wav_format::WavFormat;
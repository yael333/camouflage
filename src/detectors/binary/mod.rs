mod elf_detector;
mod exe_detector;
mod mach_detector;
mod wasm_detector;

pub use self::elf_detector::ElfDetector;
pub use self::exe_detector::ExeDetector;
pub use self::mach_detector::MachDetector;
pub use self::wasm_detector::WasmDetector;

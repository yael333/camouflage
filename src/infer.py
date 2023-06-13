import os
import argparse

# Function to parse the input string into a list of video formats
def parse_input_string(input_string):
    lines = input_string.strip().split('\n')
    video_formats = [(lines[i], lines[i+1]) for i in range(0, len(lines), 2)]
    return video_formats

# Template for .rs file
template = """use crate::Detector;
use std::error::Error;
use infer;

pub struct {struct_name}Detector;

impl Detector for {struct_name}Detector {{
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {{
        Ok(infer::{module}::{function_name}(data))
    }}
}}
"""

# CLI argument parsing
parser = argparse.ArgumentParser(description='Generate Rust modules for video format detection.')
parser.add_argument('--input', type=str, help='Input string describing video formats and their functions.')
parser.add_argument('--dir', type=str, default='./', help='Directory to write .rs files.')
args = parser.parse_args()


args.input ="""is_avi
Returns whether a buffer is AVI video data.
is_flv
Returns whether a buffer is FLV video data.
is_m4v
Returns whether a buffer is M4V video data.
is_mkv
Returns whether a buffer is MKV video data.
is_mov
Returns whether a buffer is Quicktime MOV video data.
is_mp4
Returns whether a buffer is MP4 video data.
is_mpeg
Returns whether a buffer is MPEG video data.
is_webm
Returns whether a buffer is WEBM video data.
is_wmv
Returns whether a buffer is WMV video data.
"""
video_formats = parse_input_string(args.input)
dir_path = args.dir

# Loop through the video formats and create the .rs files
mod_imports = []
mod_usages = []
for video_format, description in video_formats:
    struct_name = video_format[3:].capitalize()
    file_name = f"{struct_name.lower()}_detector.rs"
    file_content = template.format(struct_name=struct_name, function_name=video_format, module=dir_path.split('/')[1])
    file_path = os.path.join(dir_path, file_name)

    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    with open(file_path, "w") as file:
        file.write(file_content)

    mod_imports.append(f'pub use self::{struct_name.lower()}_detector::{struct_name}Detector;')
    mod_usages.append(f'(Box::new({struct_name}Detector), "{struct_name.upper()}"),')

# Update mod.rs with the import and usage statements
mod_file_path = os.path.join(dir_path, "mod.rs")
with open(mod_file_path, "w") as mod_file:
    mod_file.write('\n'.join(mod_imports) + '\n')
    mod_file.write('\n'.join(mod_usages) + '\n')

print("Finished creating .rs files.")
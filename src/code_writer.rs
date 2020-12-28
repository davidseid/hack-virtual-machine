use crate::parser;

use std::fs::File;

// CodeWriter translates VM commands into HACK Assembly Code

pub struct CodeWriter {
    file: File,
}

// Opens the output file/stream and gets ready to write into it
pub fn new(filename: &str) -> CodeWriter {
    let mut file = File::create(filename).unwrap();
    CodeWriter { file }
}

// Informs the code writer that the translation of a new VM file is started
pub fn set_filename(filename: &str) {}

// Writes the assembly code that is the translation of the given arithmetic command
pub fn write_arithmetic(command: &str) {}

// Writes the assembly code that is the translation of the given command (push or pop)
pub fn write_push_pop(command: parser::Command, segment: &str, index: usize) {}

// Closes the output file
pub fn close() {}

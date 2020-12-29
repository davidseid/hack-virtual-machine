use crate::parser;

use std::fs::File;

// CodeWriter translates VM commands into HACK Assembly Code

pub struct CodeWriter {
    hack_file: File,
    curr_vm_filename: std::option::Option<String>,
}

// Opens the output file/stream and gets ready to write into it
pub fn new(filename: &str) -> CodeWriter {
    let mut hack_file = File::create(filename).unwrap();
    CodeWriter { 
        hack_file,
        curr_vm_filename: None,
    }
}

impl CodeWriter {
    // Informs the code writer that the translation of a new VM file is started
    pub fn set_filename(&mut self, vm_filename: &str) {
        self.curr_vm_filename = Some(String::from(vm_filename));
    }

    // Writes the assembly code that is the translation of the given arithmetic command
    pub fn write_arithmetic(&self, command: &str) {}

    // Writes the assembly code that is the translation of the given command (push or pop)
    pub fn write_push_pop(&self, command: parser::Command, segment: &str, index: usize) {}

    // Closes the output file
    pub fn close(&self) {}
}



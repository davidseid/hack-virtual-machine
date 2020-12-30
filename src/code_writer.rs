use crate::parser;

use std::fs::File;
use std::io::Write;

// CodeWriter translates VM commands into HACK Assembly Code

// RAM Addresses
// 0 - 15        ~ Sixteen virtual registers
// 16 - 255      ~ Static variables (across vm files)
// 256 - 2047    ~ Stack
// 2048 - 16383  ~ Heap
// 16384 - 24575 ~ Memory mapped I/O
// 24576 - 32767 ~ Unused mem space



pub struct CodeWriter {
    hack_file: File,
    curr_vm_filename: std::option::Option<String>,
    sp: i32,
}

// Opens the output file/stream and gets ready to write into it
pub fn new(filename: &str) -> CodeWriter {
    let hack_file = File::create(filename).unwrap();
    CodeWriter { 
        hack_file,
        curr_vm_filename: None,
        sp: 256,
    }
}

impl CodeWriter {
    // Informs the code writer that the translation of a new VM file is started
    pub fn set_filename(&mut self, vm_filename: &str) {
        self.curr_vm_filename = Some(String::from(vm_filename));
    }

    // Writes the assembly code that is the translation of the given arithmetic command
    pub fn write_arithmetic(&mut self, command: &str) {
        match command {
            "add" => {
                writeln!(self.hack_file, "@SP").unwrap();
                writeln!(self.hack_file, "M=M-1").unwrap();
                writeln!(self.hack_file, "@SP").unwrap();
                writeln!(self.hack_file, "A=M").unwrap();
                writeln!(self.hack_file, "D=M").unwrap();
                writeln!(self.hack_file, "@SP").unwrap();
                writeln!(self.hack_file, "M=M-1").unwrap();
                writeln!(self.hack_file, "@SP").unwrap();
                writeln!(self.hack_file, "A=M").unwrap();
                writeln!(self.hack_file, "D=D+M").unwrap();
                writeln!(self.hack_file, "@SP").unwrap();
                writeln!(self.hack_file, "A=M").unwrap();
                writeln!(self.hack_file, "M=D").unwrap();
                writeln!(self.hack_file, "@SP").unwrap();
                writeln!(self.hack_file, "M=M+1").unwrap();
            },
            _ => (),
        }
    }

    // Writes the assembly code that is the translation of the given command (push or pop)
    pub fn write_push_pop(&mut self, command: parser::Command, segment: &str, index: usize) {
        match command {
            parser::Command::C_PUSH => {
                match segment {
                    "constant" => {
                        writeln!(self.hack_file, "@{}", index).unwrap();
                        writeln!(self.hack_file, "D=A").unwrap(); 
                        writeln!(self.hack_file, "@SP").unwrap();
                        writeln!(self.hack_file, "A=M").unwrap();
                        writeln!(self.hack_file, "M=D").unwrap();
                        writeln!(self.hack_file, "@SP").unwrap();
                        writeln!(self.hack_file, "M=M+1").unwrap();
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }

    // Closes the output file
    pub fn close(&self) {}
}



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
    // sp: i32,
    label_incrementer: i32,
}

// Opens the output file/stream and gets ready to write into it
pub fn new(filename: &str) -> CodeWriter {
    let hack_file = File::create(filename).unwrap();
    CodeWriter { 
        hack_file,
        curr_vm_filename: None,
        // sp: 256,
        label_incrementer: 0,
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
                self.write_pop_y();
                self.write_pop_x();
                writeln!(self.hack_file, "D=D+M").unwrap();
                self.write_push_result();
            },
            "sub" => {
                self.write_pop_y();
                self.write_pop_x();
                writeln!(self.hack_file, "D=M-D").unwrap();
                self.write_push_result();
            },
            "neg" =>  {
                self.write_pop_y();
                writeln!(self.hack_file, "D=-D").unwrap();
                self.write_push_result();
            },
            "eq" => {
                self.write_pop_y();
                self.write_pop_x();
                self.write_compare("JEQ");
                self.write_push_result();
            },
            "gt" => {
                self.write_pop_y();
                self.write_pop_x();
                self.write_compare("JGT");
                self.write_push_result();
            },
            "lt" => {
                self.write_pop_y();
                self.write_pop_x();
                self.write_compare("JLT");
                self.write_push_result();
            },
            "and" => {
                self.write_pop_y();
                self.write_pop_x();
                writeln!(self.hack_file, "D=D&M").unwrap();
                self.write_push_result();
            },
            "or" => {
                self.write_pop_y();
                self.write_pop_x();
                writeln!(self.hack_file, "D=D|M").unwrap();
                self.write_push_result();
            },
            "not" => {
                self.write_pop_y();
                writeln!(self.hack_file, "D=!D").unwrap();
                self.write_push_result();
            },
            _ => (),
        }
    }

    fn write_pop_y(&mut self) {
        writeln!(self.hack_file, "@SP").unwrap();
        writeln!(self.hack_file, "M=M-1").unwrap();
        writeln!(self.hack_file, "@SP").unwrap();
        writeln!(self.hack_file, "A=M").unwrap();
        writeln!(self.hack_file, "D=M").unwrap();
    }

    fn write_pop_x(&mut self) {
        writeln!(self.hack_file, "@SP").unwrap();
        writeln!(self.hack_file, "AM=M-1").unwrap();
    }

    fn write_push_result(&mut self) {
        writeln!(self.hack_file, "@SP").unwrap();
        writeln!(self.hack_file, "A=M").unwrap();
        writeln!(self.hack_file, "M=D").unwrap();
        writeln!(self.hack_file, "@SP").unwrap();
        writeln!(self.hack_file, "M=M+1").unwrap();
    }

    fn write_compare(&mut self, mnemonic: &str) {
        writeln!(self.hack_file, "D=M-D").unwrap();
        writeln!(self.hack_file, "@TRUE{}", self.label_incrementer).unwrap();
        writeln!(self.hack_file, "D;{}", mnemonic).unwrap();
        writeln!(self.hack_file, "D=0").unwrap();
        writeln!(self.hack_file, "@CONTINUE{}", self.label_incrementer).unwrap();
        writeln!(self.hack_file, "0;JMP").unwrap();
        writeln!(self.hack_file, "(TRUE{})", self.label_incrementer).unwrap();
        writeln!(self.hack_file, "D=-1").unwrap();
        writeln!(self.hack_file, "(CONTINUE{})", self.label_incrementer).unwrap();
        self.label_incrementer += 1;
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
}



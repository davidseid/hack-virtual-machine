mod code_writer;
mod parser;

use std::env;
use std::fs;

fn main() {
    println!("VM Translator starting up...");

    let source = get_source_from_command_line_arg();
    let vm_files = get_vm_files(&source);

    let hack_filename = get_hack_filename(&source);
    let mut code_writer = code_writer::new(hack_filename.as_str());

    for vm_file in vm_files {
        println!("Parsing {}", vm_file);
        let mut parser = parser::new(vm_file.as_str());

        println!("Updating code_writer to set filename {}...", vm_file);
        code_writer.set_filename(vm_file.as_str());

        println!("\nParsing debugging...\n");
        parser.advance(); // Move to first command
        while parser.has_more_commands() {
            println!("\nRaw line: {:?}", parser.get_current_command().unwrap());
            println!("Command: {:?}", parser.command_type());

            match parser.command_type() {
                parser::Command::C_RETURN => (),
                _ => println!("ARG 1: {:?}", parser.arg_1()),
            }

            match parser.command_type() {
                parser::Command::C_PUSH => println!("ARG 2: {:?}", parser.arg_2()),
                parser::Command::C_POP => println!("ARG 2: {:?}", parser.arg_2()),
                parser::Command::C_FUNCTION => println!("ARG 2: {:?}", parser.arg_2()),
                parser::Command::C_CALL => println!("ARG 2: {:?}", parser.arg_2()),
                _ => (),
            }

            // Writing
            match parser.command_type() {
                parser::Command::C_PUSH | parser::Command::C_POP => code_writer.write_push_pop(parser.command_type(), &parser.arg_1(), parser.arg_2()),
                parser::Command::C_ARITHMETIC => code_writer.write_arithmetic(&parser.arg_1()),
                _ => (),

            }

            parser.advance();
        }
    }
}

fn get_source_from_command_line_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let source = &args[1];

    println!("Source intermediary VM code: {}", source);
    String::from(source)
}

fn get_vm_files(source: &str) -> Vec<String> {
    let mut source_files = Vec::<String>::new();

    if source.contains(".vm") {
        source_files.push(String::from(source));
    } else {
        let dir_entries = fs::read_dir(&source).unwrap();

        for dir_entry in dir_entries {
            let path = dir_entry.unwrap().path();

            if path.extension().unwrap() == "vm" {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                source_files.push(String::from(format!("{}/{}", source, file_name)));
            }
        }
    }

    source_files
}

fn get_hack_filename(mut source: &str) -> String {
    if source.contains(".vm") {
        source = source.strip_suffix(".vm").unwrap();
    }
    
    String::from(format!("{}.asm", source))
}

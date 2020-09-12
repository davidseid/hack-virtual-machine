mod parser;

use std::env;
use std::fs;


fn main() {
    println!("VM Translator starting up...");

    let source = get_source_from_command_line_arg();
    let vm_files = get_vm_files(source);

    for vm_file in vm_files {
        println!("Parsing {}", vm_file);
        let parser = parser::new(vm_file.as_str());
    }

}

fn get_source_from_command_line_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let source = &args[1];

    println!("Source intermediary VM code: {}", source);
    String::from(source)
}

fn get_vm_files(source: String) -> Vec<String> {
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



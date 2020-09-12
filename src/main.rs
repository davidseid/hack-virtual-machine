use std::env;
use std::fs;

fn main() {
    println!("VM Translator starting up...");

    let vm_files = get_vm_files();

    for vm_file in vm_files {
        println!("{}", vm_file);
    }

    
}

fn get_vm_files() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let source = &args[1];

    println!("Source intermediary VM code: {}", source);

    let mut source_files = Vec::<String>::new();

    if source.contains(".vm") {
        source_files.push(String::from(source));
    } else {
        let dir_entries = fs::read_dir(&source).unwrap();

        for dir_entry in dir_entries {

            let path = dir_entry.unwrap().path();
            
            if path.extension().unwrap() == "vm" {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                source_files.push(String::from(file_name));
            }     
        }
    }

    source_files
}

use std::env;
use std::fs;

fn main() {
    println!("VM Translator starting up...");

    let args: Vec<String> = env::args().collect();
    let source = &args[1];

    println!("Source intermediary VM code: {}", source);

    // initialize array of filenames
    // if source is .vm, add it to the filenames
    // if not, get all the files in it and add it to the array

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
}

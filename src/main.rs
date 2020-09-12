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

    let mut source_files = Vec::<&str>::new();

    if source.contains(".vm") {
        source_files.push(source);
    } else {
        let paths = fs::read_dir(&source).unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    }
}

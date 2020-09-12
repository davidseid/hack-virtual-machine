use std::env;

fn main() {
    println!("VM Translator starting up...");

    let args: Vec<String> = env::args().collect();
    let source = &args[1];

    println!("Source intermediary VM code: {}", source);

    // initialize array of filenames
    // if source is .vm, add it to the filenames
    // if not, get all the files in it and add it to the array

    let mut source_files = Vec::<String>::new();
}

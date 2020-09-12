use std::env;

fn main() {
    println!("VM Translator starting up...");

    let args: Vec<String> = env::args().collect();
    let source = &args[1];

    println!("Source intermediary VM code: {}", source);
}

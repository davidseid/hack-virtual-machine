use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
pub enum Command {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
}

pub struct Parser {
    lines: Vec<String>,
    current_command_index: Option<usize>,
}

pub fn new(filename: &str) -> Parser {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        if let Ok(mut line) = line {
            println!("{}", line);

            if line.starts_with("//") {
                continue;
            }

            if line.is_empty() {
                continue;
            }

            line = line.split("//").collect::<Vec<&str>>().first().unwrap().to_string();
            line = line.split_whitespace().collect::<String>();

            lines.push(line);
        }
    }

    Parser{
        lines,
        current_command_index: None,
    }
}

impl Parser {
    pub fn get_current_command(&self) -> Option<&str> {
        match self.current_command_index {
            Some(x) => Some(self.lines[x].as_str()),
            None => None,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        match self.current_command_index {
            Some(index) => index < self.lines.len() - 1,
            None => true,
        }
    }

    pub fn advance(&mut self) {
        match self.current_command_index {
            Some(index) => {
                self.current_command_index = Some(index+1);
            },
            None => {
                self.current_command_index = Some(0);
            }
        }
    }

    pub fn command_type(&self) -> Command {
        Command::C_ARITHMETIC
    }

    pub fn arg_1(&self) {

    }

    pub fn arg_2(&self) {

    }
}    



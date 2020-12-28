use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Command {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_CALL,
    C_RETURN,
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

            if is_comment(line.as_str()) {
                continue;
            }

            if line.is_empty() {
                continue;
            }

            line = remove_comments_and_whitespace(&line);

            lines.push(line);
        }
    }

    Parser {
        lines,
        current_command_index: None,
    }
}

fn is_comment(line: &str) -> bool {
    line.starts_with("//")
}

fn remove_comments_and_whitespace(line: &str) -> String {
    line.split("//")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string()
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
                self.current_command_index = Some(index + 1);
            }
            None => {
                self.current_command_index = Some(0);
            }
        }
    }

    pub fn command_type(&self) -> Command {
        let command = self.get_current_command().unwrap();
        let first_word = command.split_whitespace().collect::<Vec<&str>>()[0];

        match first_word {
            "push" => Command::C_PUSH,
            "pop" => Command::C_POP,
            "label" => Command::C_LABEL,
            "goto" => Command::C_GOTO,
            "if-goto" => Command::C_IF,
            "function" => Command::C_FUNCTION,
            "call" => Command::C_CALL,
            "return" => Command::C_RETURN,
            _ => Command::C_ARITHMETIC,
        }
    }

    pub fn arg_1(&self) -> String {
        // Returns the first argument of the current command
        // If C_ARITHMETIC return command itself
        // Don't call this if C_RETURN
        let command = self.get_current_command().unwrap();

        match self.command_type() {
            Command::C_ARITHMETIC => command.to_string(),
            _ => command.split(" ").collect::<Vec<&str>>()[1].to_string(),
        }
    }

    pub fn arg_2(&self) -> usize {
        // Returns the second argument of the current command.
        // Only called if C_PUSH, C_POP, C_FUNCTION, C_CALL
        let command = self.get_current_command().unwrap();
        command.split_whitespace().collect::<Vec<&str>>()[2]
            .parse::<usize>()
            .unwrap()
    }
}

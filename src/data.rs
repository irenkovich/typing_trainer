use std::{
    collections::HashMap,
    fmt::{self, Display},
};

pub trait Resource {
    fn get_hints(&self) -> &HashMap<char, String>;
    fn get_content(&self) -> &Vec<String>;
}

pub struct ProgramData {
    hints: HashMap<char, String>,
    content: Vec<String>,
}

impl Resource for ProgramData {
    fn get_hints(&self) -> &HashMap<char, String> {
        &self.hints
    }

    fn get_content(&self) -> &Vec<String> {
        &self.content
    }
}

impl Default for ProgramData {
    fn default() -> Self {
        ProgramData {
            hints: load_builtin_hints(),
            content: load_builtin_content(),
        }
    }
}

enum HintLoadError {
    InvalidFormat(usize, String),
    EmptyKey(usize),
    EmptyValue(usize),
}

impl Display for HintLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidFormat(line, value) => {
                format!("invalid hint on loading. {line} line: {value}")
            }
            Self::EmptyKey(line) => format!("empty hint key on {line} line"),
            Self::EmptyValue(line) => format!("empty hint value on {line} line"),
        };

        write!(f, "{msg}")
    }
}

fn load_builtin_hints() -> HashMap<char, String> {
    let hints: Result<HashMap<char, String>, HintLoadError> = include_str!("data/hints.txt")
        .lines()
        .enumerate()
        .map(|(line_number, line_value)| {
            let (key, value) = line_value
                .split_once(":")
                .ok_or_else(|| HintLoadError::InvalidFormat(line_number, line_value.to_string()))?;

            let key = key
                .chars()
                .next()
                .ok_or(HintLoadError::EmptyKey(line_number))?;

            if value.is_empty() {
                Err(HintLoadError::EmptyValue(line_number))?;
            }

            let value = value.to_string();

            Ok((key, value))
        })
        .collect();

    hints.unwrap_or_else(|e| panic!("Error on loading hints: {e}"))
}

fn load_builtin_content() -> Vec<String> {
    let content: Vec<String> = include_str!("data/content.txt")
        .lines()
        .map(|x| x.to_lowercase())
        .collect();

    if content.is_empty() {
        panic!("No content to process, chech 'data/content.txt' file")
    }

    content
}

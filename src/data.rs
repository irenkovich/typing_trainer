use std::{collections::HashMap, io::Cursor};

use serde::Deserialize;

pub enum BuiltinMode {
    Russian,
    Greek,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    task: String,
    metadata: Option<String>,
}

impl Task {
    pub fn get_task(&self) -> &str {
        &self.task
    }

    pub fn cut_first_in_task(&mut self) {
        self.task = self.task.chars().skip(1).collect();
    }

    pub fn get_mdata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }
}

impl Default for Task {
    //dummy
    fn default() -> Self {
        Task {
            task: String::new(),
            metadata: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Hint {
    letter: char,
    hint: String,
}

pub struct ProgramData {
    hints: HashMap<char, String>,
    content: Vec<Task>,
}

impl ProgramData {
    pub fn new(mode: BuiltinMode) -> Self {
        match mode {
            BuiltinMode::Russian => ProgramData {
                content: load_builtin_content(include_bytes!("data/tasks/russian.csv")),
                hints: load_builtin_hints(include_bytes!("data/hints/russian.csv")),
            },
            BuiltinMode::Greek => ProgramData {
                content: load_builtin_content(include_bytes!("data/tasks/greek.csv")),
                hints: load_builtin_hints(include_bytes!("data/hints/greek.csv")),
            },
        }
    }
    pub fn get_hints(&self) -> &HashMap<char, String> {
        &self.hints
    }

    pub fn get_content(&self) -> &Vec<Task> {
        &self.content
    }
}

fn load_builtin_hints(hints_bytes: &[u8]) -> HashMap<char, String> {
    let reader = Cursor::new(hints_bytes);
    let mut task_reader = csv::ReaderBuilder::new()
        .delimiter(b'~')
        .from_reader(reader);
    let mut hints: HashMap<char, String> = HashMap::new();
    for res in task_reader.deserialize() {
        let hint: Hint = res.unwrap();
        hints.insert(hint.letter, hint.hint);
    }

    hints
}

fn load_builtin_content(tasks_bytes: &[u8]) -> Vec<Task> {
    let reader = Cursor::new(tasks_bytes);
    let mut task_reader = csv::ReaderBuilder::new()
        .delimiter(b'~')
        .from_reader(reader);
    let mut tasks: Vec<Task> = vec![];
    for res in task_reader.deserialize() {
        let task = res.unwrap();
        tasks.push(task);
    }

    tasks
}

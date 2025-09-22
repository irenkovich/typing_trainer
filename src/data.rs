use std::{collections::HashMap, io::Cursor};

use serde::Deserialize;

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
    pub fn get_hints(&self) -> &HashMap<char, String> {
        &self.hints
    }

    pub fn get_content(&self) -> &Vec<Task> {
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

fn load_builtin_hints() -> HashMap<char, String> {
    let russian_hints = include_bytes!("data/hints/russian.csv");
    let reader = Cursor::new(russian_hints);
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

fn load_builtin_content() -> Vec<Task> {
    let russian_tasks = include_bytes!("data/tasks/russian.csv");
    let reader = Cursor::new(russian_tasks);
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

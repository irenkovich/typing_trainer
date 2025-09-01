use std::{
    collections::HashMap,
    fmt::{self, Display},
    process::Command,
    time::Instant,
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::Rng;

fn main() {
    clear_terminal();
    let mut content = load_content();

    let hint_map = load_hints();

    let mut counter = 0;
    let mut counter_in_row = 0;
    let mut rng = rand::rng();

    let mut should_print_hint = false;

    let start = Instant::now();
    'main_loop: while !content.is_empty() {
        let rand_index = Rng::random_range(&mut rng, 0..content.len());
        let mut rand_el = content.get(rand_index).unwrap().clone();

        while !rand_el.is_empty() {
            clear_terminal();

            print_instructions(counter, counter_in_row, &rand_el);

            let target_char = rand_el.chars().next().unwrap();

            if should_print_hint {
                print_hint(&hint_map, target_char);
            }

            let user_input = process_user_input(target_char);

            match user_input {
                UserInput::Request(UserRequest::Exit) => break 'main_loop,
                UserInput::Request(UserRequest::Hint) => {
                    should_print_hint = true;
                }
                UserInput::Guess { correct } => {
                    if correct {
                        counter += 1;
                        rand_el = rand_el.chars().skip(1).collect();
                        counter_in_row += 1;
                        content.remove(rand_index);
                    } else {
                        counter_in_row = 0;
                    }

                    should_print_hint = false;
                }
            }
        }
    }
    let elapsed_secs = start.elapsed().as_secs();

    println!("You were learning for {elapsed_secs} seconds!");
    println!("Great job!");
}

enum UserRequest {
    Hint,
    Exit,
}

enum UserInput {
    Request(UserRequest),
    Guess { correct: bool },
}

fn process_user_input(target_char: char) -> UserInput {
    listen_keys_pressing();

    let result = if let Event::Key(key_event) = event::read().unwrap() {
        match key_event.code {
            KeyCode::Esc => UserInput::Request(UserRequest::Exit),
            KeyCode::Char(c) => {
                let guess = c.to_lowercase().next().unwrap();
                UserInput::Guess {
                    correct: guess == target_char,
                }
            }
            KeyCode::Backspace => UserInput::Request(UserRequest::Hint),
            _ => UserInput::Guess { correct: false },
        }
    } else {
        UserInput::Guess { correct: false }
    };

    stop_listening_keys_pressing();

    result
}

fn stop_listening_keys_pressing() {
    disable_raw_mode()
        .unwrap_or_else(|_| panic!("Unexpected error on stop waiting for keys pressing"));
}

fn listen_keys_pressing() {
    enable_raw_mode().unwrap_or_else(|_| panic!("unexpected error on waiting for key pressing"));
}

fn print_hint(hint_map: &HashMap<char, String>, target_char: char) {
    let hint = hint_map
        .get(&target_char)
        .map_or(String::from("No hint for this :("), |x| x.clone());
    println!("{hint}");
}

fn print_instructions(counter: i32, counter_in_row: i32, rand_el: &String) {
    let sweet_words = get_sweet_words_for_typer(counter_in_row);
    println!("Press esc for exit, backspace for hint.");
    println!();
    println!("Scores: {counter}. {sweet_words}");
    println!();
    println!("{rand_el}");
}

fn get_sweet_words_for_typer(counter_in_row: i32) -> String {
    match counter_in_row {
        3..=50 => format!("Wow! {counter_in_row} in a row! Let's do it!"),
        51..=100 => format!("{counter_in_row} in a row! You know something, don't you?"),
        101..=200 => format!("Proffesional typer! {counter_in_row} in a row!"),
        201..=500 => format!("SOMEONE STOP THIS KEYS-REAPER! {counter_in_row} IN A ROW!"),
        501..30000 => {
            format!(
                "Don't you want to relax a bit, perfectionsim-cultist? {counter_in_row} in a row."
            )
        }
        _ => String::new(),
    }
}

fn load_content() -> Vec<String> {
    let content: Vec<String> = include_str!("data/content.txt")
        .lines()
        .map(|x| x.to_lowercase())
        .collect();

    if content.is_empty() {
        panic!("No content to process, chech 'data/content.txt' file")
    }

    content
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

fn load_hints() -> HashMap<char, String> {
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

fn clear_terminal() {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()
    } else {
        Command::new("clear").status()
    };

    status.unwrap_or_else(|e| panic!("Error on terminal clearing: {e}"));
}

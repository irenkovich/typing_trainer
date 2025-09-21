use std::{collections::HashMap, process::Command, time::Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::Rng;

use crate::{
    data::{ProgramData, Resource},
    state::State,
};

pub mod data;
pub mod state;

fn main() {
    clear_terminal();
    let program_data = ProgramData::default();

    let mut content = program_data.get_content().clone();

    let hint_map = program_data.get_hints();

    let mut state = State::new();
    let mut rng = rand::rng();

    let start = Instant::now();
    'main_loop: while !content.is_empty() {
        let rand_index = Rng::random_range(&mut rng, 0..content.len());
        state.set_current_task(content.get(rand_index).unwrap());

        while state.task_in_progress() {
            clear_terminal();

            print_instructions(&state);

            let target_char = state.current_task().chars().next().unwrap();

            if state.print_hint() {
                print_hint(hint_map, target_char);
            }

            let user_input = process_user_input(target_char);

            match user_input {
                UserInput::Request(UserRequest::Exit) => break 'main_loop,
                UserInput::Request(UserRequest::Hint) => {
                    state.enable_hint();
                }
                UserInput::Guess { correct } => {
                    if correct {
                        state.process_right_guess();
                    } else {
                        state.process_incorrect_guess();
                    }

                    state.disable_hint();
                }
            }
        }
        content.remove(rand_index);
    }
    let elapsed_secs = start.elapsed().as_secs();

    println!(
        "You were learning for {} seconds!",
        elapsed_secs.to_string().green()
    );
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
        .map_or(String::from("No hint for this :("), |x| x.clone())
        .green();
    println!("{hint}");
}

fn print_instructions(state: &State) {
    let counter_in_row = state.in_row();
    let counter = state.counter();
    let task = state.current_task();

    let sweet_words = get_sweet_words_for_typer(counter_in_row);
    println!(
        "Press {} for exit, {} for hint.",
        "ESC".yellow(),
        "Backspace".yellow()
    );
    println!();
    println!("Scores: {}. {sweet_words}", counter.to_string().green());
    println!();

    let fisrt_char = if state.last_guess_err() {
        task.chars().next().unwrap().on_red()
    } else {
        task.chars().next().unwrap().on_blue()
    };
    let other_part: String = task.chars().skip(1).collect();
    println!("{fisrt_char}{other_part}");
}

fn get_sweet_words_for_typer(counter_in_row: u32) -> String {
    let counter_in_row_str = counter_in_row.to_string().green();
    match counter_in_row {
        3..=50 => format!("Wow! {counter_in_row_str} in a row! Let's do it!"),
        51..=100 => format!("{counter_in_row_str} in a row! You know something, don't you?"),
        101..=200 => format!("Proffesional typer! {counter_in_row_str} in a row!"),
        201..=500 => format!("SOMEONE STOP THIS KEYS-REAPER! {counter_in_row_str} IN A ROW!"),
        501..30000 => {
            format!(
                "Don't you want to relax a bit, perfectionsim-cultist? {counter_in_row_str} in a row."
            )
        }
        _ => String::new(),
    }
}

fn clear_terminal() {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()
    } else {
        Command::new("clear").status()
    };

    status.unwrap_or_else(|e| panic!("Error on terminal clearing: {e}"));
}

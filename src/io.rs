use std::{collections::HashMap, process::Command, time::Duration};

use crossterm::{
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::state::State;

pub fn print_hint(hint_map: &HashMap<char, String>, target_char: char) {
    let hint = hint_map
        .get(&target_char.to_lowercase().next().unwrap())
        .map_or(String::from("No hint for this :("), |x| x.clone())
        .green();
    println!("{hint}");
}

pub fn print_instructions(state: &State) {
    let counter_in_row = state.in_row();
    let counter = state.counter();
    let task = state.current_task().get_task();

    let sweet_words = get_sweet_words_for_typer(counter_in_row);
    println!(
        "Press {} for exit, {} for hint.",
        "ESC".yellow(),
        "Backspace".yellow()
    );
    println!();
    println!("Scores: {}. {sweet_words}", counter.to_string().green());
    println!();

    let mdata = state.current_task().get_mdata();
    if let Some(md) = mdata {
        println!("{md}");
        println!();
    }

    let fisrt_char = if state.last_guess_err() {
        task.chars().next().unwrap().on_red()
    } else {
        task.chars().next().unwrap().on_blue()
    };
    let other_part: String = task.chars().skip(1).collect();
    println!("{fisrt_char}{other_part}");
}

pub fn get_sweet_words_for_typer(counter_in_row: u32) -> String {
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

pub fn stop_listening_keys_pressing() {
    disable_raw_mode()
        .unwrap_or_else(|_| panic!("Unexpected error on stop waiting for keys pressing"));
}

pub fn listen_keys_pressing() {
    enable_raw_mode().unwrap_or_else(|_| panic!("unexpected error on waiting for key pressing"));
}

pub fn clear_terminal() {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()
    } else {
        Command::new("clear").status()
    };

    status.unwrap_or_else(|e| panic!("Error on terminal clearing: {e}"));
}

pub fn say_goodbye(duration: Duration) {
    println!(
        "You were learning for {} seconds!",
        duration.as_secs().to_string().green()
    );
    println!("Great job!");
}

use crossterm::event::{self, Event, KeyCode};

use crate::{data::BuiltinMode, io, state::State};

pub enum UserRequest {
    Hint,
    Exit,
}

pub enum UserInput {
    Request(UserRequest),
    Guess { correct: bool },
}

pub fn choose_mode() -> BuiltinMode {
    io::listen_keys_pressing();

    let result = loop {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char(var) => match var {
                    '1' => break BuiltinMode::Russian,
                    '2' => break BuiltinMode::Greek,
                    _ => continue,
                },
                _ => continue,
            }
        }
    };
    io::stop_listening_keys_pressing();
    result
}

pub fn process_user_input(state: &State) -> UserInput {
    io::listen_keys_pressing();

    let result = if let Event::Key(key_event) = event::read().unwrap() {
        match key_event.code {
            KeyCode::Esc => UserInput::Request(UserRequest::Exit),
            KeyCode::Char(guess) => UserInput::Guess {
                correct: guess == state.get_expected_char(),
            },
            KeyCode::Backspace => UserInput::Request(UserRequest::Hint),
            _ => UserInput::Guess { correct: false },
        }
    } else {
        UserInput::Guess { correct: false }
    };

    io::stop_listening_keys_pressing();

    result
}

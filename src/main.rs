use std::time::Instant;

use rand::Rng;

use crate::{
    data::{BuiltinMode, ProgramData},
    state::State,
    user::{UserInput, UserRequest, choose_mode},
};

pub mod data;
pub mod io;
pub mod state;
pub mod user;

fn main() {
    io::clear_terminal();

    println!("Choose mode:\n1. Russian\n2. Greek\n");
    let mode: BuiltinMode = choose_mode();
    let program_data = ProgramData::new(mode);

    let mut content = program_data.get_content().clone();

    let hint_map = program_data.get_hints();

    let mut rng = rand::rng();

    let mut state = State::new();

    let start = Instant::now();
    'main_loop: while !content.is_empty() {
        let rand_index = Rng::random_range(&mut rng, 0..content.len());
        let task = content.get(rand_index).unwrap().clone();
        //prevent duplicates
        content.remove(rand_index);

        state.set_current_task(task);

        while state.task_in_progress() {
            io::clear_terminal();

            io::print_instructions(&state);

            if state.print_hint() {
                io::print_hint(hint_map, state.get_expected_char());
            }

            let user_input = user::process_user_input(&state);

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
    }

    let elapsed = start.elapsed();
    io::say_goodbye(elapsed);
}

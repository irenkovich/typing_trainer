use std::time::Instant;

use rand::Rng;

use crate::{
    data::{ProgramData, Resource},
    state::State,
    user::UserInput,
    user::UserRequest,
};

pub mod data;
pub mod io;
pub mod state;
pub mod user;

fn main() {
    io::clear_terminal();
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
        // prevent duplicates
        content.remove(rand_index);
    }

    let elapsed = start.elapsed();
    io::say_goodbye(elapsed);
}

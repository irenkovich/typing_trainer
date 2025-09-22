use crate::data::Task;

pub struct State {
    counter: u32,
    in_row: u32,
    print_hint: bool,
    current_task: Task,
    last_guess_err: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            counter: 0,
            in_row: 0,
            print_hint: false,
            current_task: Task::default(),
            last_guess_err: false,
        }
    }

    pub fn set_current_task(&mut self, next: Task) {
        self.current_task = next;
    }

    pub fn task_in_progress(&self) -> bool {
        !&self.current_task.get_task().is_empty()
    }

    pub fn enable_hint(&mut self) {
        self.print_hint = true;
    }

    pub fn disable_hint(&mut self) {
        self.print_hint = false;
    }

    pub fn process_right_guess(&mut self) {
        self.counter += 1;
        self.in_row += 1;
        self.last_guess_err = false;
        self.current_task.cut_first_in_task();
    }

    pub fn process_incorrect_guess(&mut self) {
        self.last_guess_err = true;
        self.in_row = 0;
    }

    pub fn counter(&self) -> u32 {
        self.counter
    }

    pub fn in_row(&self) -> u32 {
        self.in_row
    }

    pub fn print_hint(&self) -> bool {
        self.print_hint
    }

    pub fn current_task(&self) -> &Task {
        &self.current_task
    }

    pub fn last_guess_err(&self) -> bool {
        self.last_guess_err
    }

    pub fn get_expected_char(&self) -> char {
        self.current_task.get_task().chars().next().unwrap()
    }
}

use crate::utils::terminal::{clear_screen, show_menu};

mod control_flow;
mod enums;
mod pi_by_probability;
mod quiz;


pub fn run() {
    loop {

        let items = [
            "Enums",
            "Control Flow",
            "Quiz",
            "π by Probability",
        ];

        let user_input = show_menu("Chapter 06 - Enums and Pattern Matching", &items, false);

        clear_screen();

        match user_input {
            1 => enums::run(),
            2 => control_flow::run(),
            3 => quiz::run(),
            4 => pi_by_probability::run(),
            _ => break,
        }
    }
}

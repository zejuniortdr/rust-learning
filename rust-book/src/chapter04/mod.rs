use crate::utils::terminal::{clear_screen, show_menu};

mod ownership;
mod quiz;


pub fn run() {
    loop {

        let items = [
            "Ownership",
            "Quiz",
        ];

        let user_input = show_menu("Chapter 04 - Understading Ownership", &items, false);

        clear_screen();

        match user_input {
            1 => ownership::run(),
            2 => quiz::run(),
            _ => break,
        }
    }
}

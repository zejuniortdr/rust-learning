use crate::utils::terminal::{clear_screen, show_menu};

mod guessing_game;

pub fn run() {
    loop {

        let items = [
            "Guessing Game",
        ];

        let user_input = show_menu("Chapter 02 - Guessing Game", &items, false);

        clear_screen();

        match user_input {
            1 => guessing_game::run(),
            _ => break,
        }
    }
}

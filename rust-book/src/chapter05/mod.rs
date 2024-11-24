use crate::utils::terminal::{clear_screen, show_menu};

mod rectangles;
mod quiz;


pub fn run() {
    loop {

        let items = [
            "Rectangles",
            "Quiz",
        ];

        let user_input = show_menu("Chapter 05 - Using Structs to Structure Related Data", &items, false);

        clear_screen();

        match user_input {
            1 => rectangles::run(),
            2 => quiz::run(),
            _ => break,
        }
    }
}

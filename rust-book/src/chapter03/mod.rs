use crate::utils::terminal::{clear_screen, show_menu};

mod challenges;
mod common_concepts;
mod functions;
mod loops;
mod quiz;


pub fn run() {
    loop {

        let items = [
            "Common concepts",
            "Functions",
            "Loops",
            "Challenges",
            "Quiz",
        ];

        let user_input = show_menu("Chapter 03 - Common Concepts", &items, false);

        clear_screen();

        match user_input {
            1 => common_concepts::run(),
            2 => functions::run(),
            3 => loops::run(),
            4 => challenges::run(),
            5 => quiz::run(),
            _ => break,
        }
    }
}

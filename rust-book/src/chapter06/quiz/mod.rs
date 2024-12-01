use crate::utils::terminal::{clear_screen, show_menu};

mod quiz1;
mod quiz2;
mod quiz3;
mod quiz4;


pub fn run() {
    loop {

        let items = [
            "1",
            "2",
            "3",
            "4",
        ];

        let user_input = show_menu("Chapter 06 - Quiz", &items, false);

        clear_screen();

        match user_input {
            1 => quiz1::run(),
            2 => quiz2::run(),
            3 => quiz3::run(),
            4 => quiz4::run(),
            _ => break,
        }
    }
}

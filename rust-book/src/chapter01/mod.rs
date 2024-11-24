use crate::utils::terminal::{clear_screen, show_menu};

mod hello_cargo;
mod hello_world;

pub fn run() {
    loop {

        let items = [
            "Hello World!",
            "Hello World with Cargo Watch!",
        ];

        let user_input = show_menu("Chapter 01", &items, false);

        clear_screen();

        match user_input {
            1 => hello_world::hello_world(),
            2 => hello_cargo::hello_cargo(),
            _ => break,
        }
    }
}

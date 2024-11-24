mod chapter01;
mod chapter02;
mod chapter03;
mod chapter04;
mod chapter05;
mod chapter06;
mod utils;

use utils::terminal::{show_menu,terminate};


fn main() {
    loop {
        let items = [
            "Chapter 01 - Hello World!",
            "Chapter 02 - Guessing Game",
            "Chapter 03 - Common Concepts",
            "Chapter 04 - Understanding Ownership",
            "Chapter 05 - Using Structs to Structure Related Data",
            "Chapter 06 - Enums and Pattern Matching",
        ];
        let chosen_option= show_menu("Main", &items, true);

        match chosen_option {
            1 => chapter01::run(),
            2 => chapter02::run(),
            3 => chapter03::run(),
            4 => chapter04::run(),
            5 => chapter05::run(),
            6 => chapter06::run(),
            _ => terminate(),
        }
    }

}

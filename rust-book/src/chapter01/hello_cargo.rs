use crate::utils::terminal::wait_for_enter;

pub fn hello_cargo() {
    println!("Hello, world!");
    println!("Hello, world on Cargo Watch!");
    println!("Run this again with cargo watch instead of cargo run to see the differences and change some file.");
    wait_for_enter();
}

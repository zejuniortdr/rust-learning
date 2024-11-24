use crate::utils::terminal::wait_for_enter;

pub fn hello_world() {
    println!("Hello, world!");
    wait_for_enter();
}

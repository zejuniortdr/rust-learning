fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // DONE: Fix the function call.
    // call_me();
    // ^^^^^^^-- argument #1 of type `u8` is missing

    call_me(10);

}

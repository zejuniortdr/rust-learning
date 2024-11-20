// DONE: Fix the compiler error.
fn main() {
    // let x = 3;
    // - first assignment to `x`

    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    // ^^^^^ cannot assign twice to immutable variable

    println!("Number {x}");
}

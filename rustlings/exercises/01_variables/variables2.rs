fn main() {
    // DONE: Change the line below to fix the compiler error.
    // let x;

    //let x:i32;
    // - binding declared here but left uninitialized

    let x:i32 = 11;

    if x == 10 {
        // -- type must be known at this point
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}

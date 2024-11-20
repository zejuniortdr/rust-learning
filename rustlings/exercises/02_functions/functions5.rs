// DONE: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    // num * num;
    //       - help: remove this semicolon to return this value
    // implicitly returns `()` as its body has no tail or `return` expression
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}

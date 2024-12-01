use crate::utils::terminal::wait_for_enter;


// fn f_original(x) {
//     println!("{x}");
// }
// Context: A function must declare the types of its parameters.
// = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
// help: if this is a `self` type, give it a parameter name
//   |
// 5 | fn f(self: x) {
//   |      +++++
// help: if this is a parameter name, give it a type
//   |
// 5 | fn f(x: TypeName) {
//   |       ++++++++++
// help: if this is a type, explicitly ignore the parameter name
//   |
// 5 | fn f(_: x) {
//   |      ++




// SOLVED
fn f(x: i32) -> i32 {
    x + 1
}

pub fn run() {
    println!("f(0) = {}", f(0));
    wait_for_enter()
}

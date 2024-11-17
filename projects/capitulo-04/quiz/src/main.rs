// fn main() {
//     let s = String::from("hello");
//     let s2;
//     let b = false;
//     if b {
//         s2 = s;
//     }
//     println!("{}", s);
// }


fn move_a_box(b: Box<i32>) {
    // This space intentionally left blank
}

fn main() {
    let b = Box::new(0);
    move_a_box(b);
    println!("{}", b);
}

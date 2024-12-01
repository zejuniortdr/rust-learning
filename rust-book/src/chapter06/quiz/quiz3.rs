use crate::utils::terminal::wait_for_enter;

#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String)
}

// fn run_original() {
//     let x = Either::Right(String::from("Hello world"));
//     let value = match x {
//         Either::Left(n) => n,
//         Either::Right(s) => s.len()
//     };

//     println!("{x:?} {value}");
//     // ^^^^^ value borrowed here after partial move

//     wait_for_enter();
// }

// SOLVED
pub fn run() {
    let x = Either::Right(String::from("Hello world"));
    let value = match x {
        Either::Left(n) => n,
        Either::Right(ref s) => s.len()
    };

    println!("{x:?} {value}");
    // ^^^^^ value borrowed here after partial move

    wait_for_enter();
}

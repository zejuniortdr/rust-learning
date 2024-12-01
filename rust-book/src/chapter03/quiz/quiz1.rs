use crate::utils::terminal::wait_for_enter;

pub fn run() {
    // let mut x: u32 = 1;
    // variable does not need to be mutable

    // SOLVED
    let x:u32 = 1;

    {
        // let mut x = x;
        // warning: variable `x` is assigned to, but never used
        // --> src/chapter03/quiz/quiz1.rs:11:17
        // |
        // 11 |         let mut x = x;
        // |                 ^
        // |
        // = note: consider using `_x` instead
        // = note: `#[warn(unused_variables)]` on by default


        // x += 2;
        // warning: value assigned to `x` is never read
        // --> src/chapter03/quiz/quiz1.rs:12:9
        // |
        // 12 |         x += 2;
        // |         ^
        // |
        // = help: maybe it is overwritten before being read?
        // = note: `#[warn(unused_assignments)]` on by default

        // SOLVED
        let mut _x = x;
        _x += 2;
    }
    println!("{x}");

    wait_for_enter()
}

use crate::utils::terminal::wait_for_enter;

enum Location {
    Point(i32),
    Range(i32, i32)
}
// warning: field `0` is never read
//  --> src/chapter06/quiz/quiz1.rs:5:11
//   |
// 5 |     Range(i32, i32)
//   |     ----- ^^^
//   |     |
//   |     field in this variant
//   |
//   = note: `#[warn(dead_code)]` on by default
// help: consider changing the field to be of unit type to suppress this warning while preserving the field numbering, or remove the field
//   |
// 5 |     Range((), i32)
//   |


// pub fn run_original() {
//     let l: Location = Location::Range(0, 5);
//     let n = match l {
//         Location::Point(_) => -1,
//         // --------------------- matches all the relevant values

//         Location::Range(_, n) => n,
//         // ^^^^^^^^^^^^^^^^^^^^^ no value can reach this

//         Location::Range(0, _) => 0,
//         // unreachable pattern
//         // `#[warn(unreachable_patterns)]` on by defaultrustcClick for full compiler diagnostic
//         // quiz.rs(16, 9): matches all the relevant values

//         _ => -2
//         // ^^^^^^^^^^^^^^^^^^^^^ no value can reach this
//     };

//     println!("{n}");
//     wait_for_enter();
// }


pub fn run() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::Point(_) => -1,
        // --------------------- matches all the relevant values

        Location::Range(_, n) => n,
        // ^^^^^^^^^^^^^^^^^^^^^ no value can reach this
    };

    println!("{n}");
    wait_for_enter();
}

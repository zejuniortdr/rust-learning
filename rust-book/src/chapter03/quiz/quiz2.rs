use crate::utils::terminal::wait_for_enter;

pub fn run() {
    let message = "The temperature today is :";

    // let x = [message, 100];
    //                   ^^^ expected `&str`, found integer

    // SOLVED
    let x = [message, "100"];

    println!("{} {}", x[0], x[1]);

    let t = ([1;2], [3;4]);

    // let (a,b) = t;
    // warning: unused variable: `b`
    // --> src/chapter03/quiz/quiz2.rs:15:12
    // |
    // 15 |     let (a,b) = t;
    // |            ^ help: if this is intentional, prefix it with an underscore: `_b`

    // SOLVED
    let (a,_) = t;

    println!("{}", a[0] + t.1[0]);

    wait_for_enter()
}

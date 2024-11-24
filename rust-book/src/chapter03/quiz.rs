use crate::utils::terminal::wait_for_enter;

pub fn run() {
    // let mut x: u32 = 1;
    // {
    //     let mut x = x;
    //     x += 2;
    // }
    // println!("{x}");

    // let x: fsize = 2.0;
    // println!("{x}");


    // let message = "The temperature today is :";
    // let x = [message, 100]
    // println!("{} {}", x[0], x[1]);

    // let t = ([1;2], [3;4]);
    // let (a,b) = t;
    // println!("{}", a[0] + t.1[0]);

    let mut x = 0;
    'a: loop {
        x += 1;
        println!("{x}");
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }

    wait_for_enter()
}

// fn f(x: i32) -> i32 { x + 1 }
// fn f(x) {
//     println!("{x}");
// }

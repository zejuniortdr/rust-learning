use crate::utils::terminal::wait_for_enter;

pub fn run() {
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

use crate::utils::terminal::wait_for_enter;

fn decr_twice_v1(n: u32) -> Option<u32> {
    match n {
        0 => None,
        1 => None,
        n2 => Some(n2 - 2)
    }
}


 fn decr_twice_v2(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        None
    } else {
        Some(n - 2)
    }
}

pub fn run() {
    println!("{:?} {:?}", decr_twice_v1(0), decr_twice_v2(0));
    println!("{:?} {:?}", decr_twice_v1(1), decr_twice_v2(1));
    println!("{:?} {:?}", decr_twice_v1(3), decr_twice_v2(3));

    wait_for_enter();
}

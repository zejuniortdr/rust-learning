use crate::utils::terminal::wait_for_enter;


fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn add_suffix(mut name:String) -> String {
    name.push_str(" Jr.");
    name
}

pub fn run() {
    let x = true;
    read(x);

    let mut name = String::from("John");
    name = add_suffix(name);
    println!("{name}");

    wait_for_enter();
}

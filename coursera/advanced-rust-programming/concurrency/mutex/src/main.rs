use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }

    println!("Counter: {:?}", counter);
}

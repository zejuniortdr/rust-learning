use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number from the spawned thread: {}", i);
        }
    });

    for i in 1..5 {
        println!("Hi number from the main thread: {}", i);
    }

    handle.join().unwrap();
}

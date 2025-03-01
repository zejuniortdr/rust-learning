use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {

    let counter = AtomicUsize::new(0);

    let increment = || {

        counter.fetch_add(1, Ordering::SeqCst);

    };

    increment();

    increment();

    println!("Counter: {}", counter.load(Ordering::SeqCst));

}

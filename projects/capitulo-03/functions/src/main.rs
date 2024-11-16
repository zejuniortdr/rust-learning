fn main() {
    println!("Hello, world!");
    another_function();
    let fat = 20;
    println!("{}! = {}", fat, factorial_recursive(fat));

    println!("Is 4 prime? {}", is_prime(4));
    println!("Is 9 prime? {}", is_prime(9));
    println!("Is 13 prime? {}", is_prime(13));
    println!("Is 100 prime? {}", is_prime(100));

    println!("Is 6 a perfect number? {}", is_perfect_number(6));
    println!("Is 28 a perfect number? {}", is_perfect_number(28));
    println!("Is 12 a perfect number? {}", is_perfect_number(12));
}

fn another_function() {
    println!("Another function!");
}


// A function must declare the types of its parameters.
fn factorial_recursive(x: u64) -> u64 {
    if x == 0 {
        1
        // sem o ponto e virgula significa o retorno da função
    } else {
        x * factorial_recursive(x-1)
    }
}

fn is_prime(x: u32) -> bool {
    if x <= 1 {
        false
    } else if x <= 3 {
        true
    } else if x % 2 == 0 || x % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= x {
            if x % i == 0 || x % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}


fn is_perfect_number(x: u32) -> bool {
    let sum = (1..x).filter(|&i| x % i == 0).sum();
    x == sum
}

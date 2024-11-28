use num::integer::{gcd};
use rand::Rng;
use std::f64::consts::PI;
use std::time::Instant;
use std::io;

use crate::utils::terminal::wait_for_enter;

pub fn run(){
    let tries = read_number("Number of tries to stop", 500) as f64;
    let decimal_places_precision = read_number("Decimal places precision", 1) as i32;
    let random_start = read_number("Random Start at", 1);
    let random_end = read_number("Random End at", 100);


    let mut i = 0.0;
    let mut coprimes = 0.0;
    let mut pi_found = 0.0;
    let mut diff:f64 = PI-pi_found;

    let precision = 10.0f64.powi(-decimal_places_precision-1);
    println!("Searching for precision of {precision} of PI...");

    let start = Instant::now();

    while diff > precision {
        i += 1.0;

        let n1 = rand::thread_rng().gen_range(random_start..=random_end);
        let n2 = rand::thread_rng().gen_range(random_start..=random_end);

        if gcd(n1, n2) == 1 {
            coprimes += 1.0;
        }

        let coprimes:f64 = coprimes;
        let x = coprimes / i;

        pi_found = (6.0/x).sqrt();
        diff = (PI - pi_found).abs();

        if i >= tries {
            println!("Hard stop after: {tries} tries");
            break;
        }

    };
    let duration = start.elapsed();

    println!("PI found: {pi_found}");
    println!("PI constant: {PI}");
    println!("Numbers of tries: {i}");
    println!("DIFF: {}", diff);
    println!("Precision found: {} decimal places", get_precision(diff));
    println!("Execution time: {:?}", duration);

    wait_for_enter();
}


fn read_number(label:&str, default:u32) -> u32 {
    println!("Input {label}:");
    // cria uma string mutável vazia
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    // Faz o parse da entrada para u32
    // Caso ocorra algum erro, o programa continua a perguntar o número.
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => default,
    };
    number
}


fn get_precision(n:f64) -> u32 {
    let mut number = n;
    let mut precision:u32 = 0;
    while number * 10.0 < 1.0 {
        precision += 1;
        number = number * 10.0;
    }
    return precision;
}

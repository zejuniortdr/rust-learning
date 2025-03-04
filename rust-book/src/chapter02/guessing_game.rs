use rand::Rng; // import responsável pelo Random
use std::cmp::Ordering; // funciona similar ao switch/case de algumas linguagens
use std::io;

use crate::utils::terminal::wait_for_enter; // import para capturar input do terminal

pub fn run() {
    println!("Guess the number!");

    // gera um numero aleatório entre 1 e 100 incluindo os extremos
    let secret_number = rand::rng().random_range(1..=100);


    let tries = 10;
    let mut user_tries = 0;

    loop {
        println!("{user_tries}/{tries} Please input your guess.");

        // cria uma string mutável vazia
        let mut guess = String::new();

        // lê uma entrada do terminal e salva na variável guess.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Faz o parse da entrada para u32
        // Caso ocorra algum erro, o programa continua a perguntar o número.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // compara o número escolhido com o número secreto
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                user_tries += 1;
                if user_tries == tries {
                    println!("You've lost the game! The number was: {}", secret_number);
                    break;
                }
            }
            Ordering::Greater => {
                println!("Too big!");
                user_tries += 1;
                if user_tries == tries {
                    println!("You've lost the game! The number was: {}", secret_number);
                    break;
                }
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    wait_for_enter();
}

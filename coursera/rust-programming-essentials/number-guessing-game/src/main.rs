use rand; // import responsável pelo Random
use std::cmp::Ordering; // funciona similar ao switch/case de algumas linguagens
use std::io;


fn main() {
    let min_number = 1;
    let max_number = 100;

    println!("Guess the number between {} and {}!", min_number, max_number);

    // gera um numero aleatório entre 1 e 100 incluindo os extremos
    let secret_number = rand::random_range(min_number..=max_number);

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
            Err(_) => {
                println!("Please type a number!");
                user_tries += 1;
                continue;
            }
        };

        // compara o número escolhido com o o range de possibilidades
        if guess < min_number || guess > max_number {
            println!("The number must be between {} and {}!", min_number, max_number);
            user_tries += 1;
            continue;
        }


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
    // wait_for_enter();
}

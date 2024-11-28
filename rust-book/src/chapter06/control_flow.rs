use crate::utils::terminal::wait_for_enter;

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    HalfDollar,
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum CoinState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    HalfDollar,
}


pub fn run() {
    let coins: [Coin; 5] = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter, Coin::HalfDollar];
    for coin in coins {
        println!("{:?} = {} cents", coin, value_in_cents(&coin));
    }

    let coins_state: [CoinState; 5] = [
        CoinState::Penny,
        CoinState::Nickel,
        CoinState::Dime,
        CoinState::Quarter(UsState::Alaska),
        CoinState::HalfDollar,
    ];

    for coin_state in coins_state {
        println!("{:?} = {} cents", coin_state, value_in_cents_by_state(&coin_state));
    }

    println!("\nMatching with Option<T>");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}, six: {:?}, none:{:?}", five, six, none);

    wait_for_enter();

}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::HalfDollar => 50,
    }
}


fn value_in_cents_by_state(coin: &CoinState) -> u8 {
    match coin {
        CoinState::Penny => 1,
        CoinState::Nickel => 5,
        CoinState::Dime => 10,
        CoinState::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
        CoinState::HalfDollar => 50,
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n + 1),
        None => None,
    }
}

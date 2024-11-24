
use crate::utils::terminal::{print_header, wait_for_enter};

pub fn run() {
    println!("100 F = {} C", f_to_c(100.0));
    println!("30 c = {} F", c_to_f(30.0));
    println!("3rd Fibonnaci = {}", nth_fibbonatti(3));
    println!("3rd Fibonnaci Recursive = {}", nth_fibbonatti_rec(10));
    println!("10rd Fibonnaci = {}", nth_fibbonatti(3));
    println!("10rd Fibonnaci Recursive = {}", nth_fibbonatti_rec(10));

    the_twelve_days_of_christmas();

    wait_for_enter();
}

fn f_to_c(degree:f64) -> f64 {
    (degree - 32.0) / 1.8
}

fn c_to_f(degree:f64) -> f64 {
    (degree * 1.8) + 32.0
}

fn nth_fibbonatti_rec(n:u32) -> u32{
    if n <= 1 {
        n
    } else {
        nth_fibbonatti_rec(n-1) + nth_fibbonatti_rec(n-2)
    }
}


fn nth_fibbonatti(n:u32) -> u32 {
    let mut current = 1;
    let mut previous = 1;

    for _ in 1..n {
        let next = current + previous;
        current = previous;
        previous = next;
    }
    current
}


fn the_twelve_days_of_christmas() {
    let lyrics = [
        ("first", "a partridge in a pear tree." ),
        ("second", "Two turtle doves," ),
        ("third", "Three French hens," ),
        ("fourth", "Four calling birds," ),
        ("fifth", "Five golden rings," ),
        ("sixth", "Six geese a-laying," ),
        ("seventh", "Seven swans a-swimming," ),
        ("eighth", "Eight maids a-milking," ),
        ("eighth", "Eight maids a-milking," ),
        ("ninth", "Nine ladies dancing," ),
        ("tenth", "Ten lords a-leaping," ),
        ("eleventh", "Eleven pipers piping," ),
        ("twelfth", "Twelve drummers drumming," ),
    ];
    let mut index = 1;

    let title = "The Twelve Days of Christmas (song)";
    print_header(&title);

    for verse in lyrics {
        println!("On the {} day of Christmas, my true love sent to me:", verse.0);

        for i in (1..index+1).rev() {
            match i {
                1 => println!("a partridge in a pear tree."),
                2 => println!("two turtle doves"),
                3 => println!("three French hens"),
                4 => println!("four calling birds"),
                5 => println!("five golden rings"),
                6 => println!("six geese a-laying"),
                7 => println!("seven swans a-swimming"),
                8 => println!("eight maids a-milking"),
                9 => println!("nine ladies dancing"),
                10 => println!("ten lords a-leaping"),
                11 => println!("eleven pipers piping"),
                12 => println!("twelve drummers drumming"),
                _ => (),
            }
        }
        println!("\n");
        index += 1;
    }
}
